using System;
using System.Collections.Generic;
using System.Linq;
using System.Linq.Expressions;
using System.Net;
using System.Reactive.Disposables;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Reactive.Threading.Tasks;
using System.Reflection;
using System.Threading;
using System.Threading.Tasks;
using Firebase.Database.Extensions;
using Firebase.Database.Offline.Internals;
using Firebase.Database.Query;
using Firebase.Database.Streaming;
using Newtonsoft.Json;

namespace Firebase.Database.Offline
{
    /// <summary>
    ///     The real-time Database which synchronizes online and offline data.
    /// </summary>
    /// <typeparam name="T"> Type of entities. </typeparam>
    public class RealtimeDatabase<T> : IDisposable where T : class
    {
        private readonly ChildQuery childQuery;
        private readonly string elementRoot;
        private readonly FirebaseCache<T> firebaseCache;
        private readonly InitialPullStrategy initialPullStrategy;
        private readonly bool pushChanges;
        private readonly StreamingOptions streamingOptions;
        private readonly Subject<FirebaseEvent<T>> subject;
        private FirebaseSubscription<T> firebaseSubscription;

        private bool isSyncRunning;
        private IObservable<FirebaseEvent<T>> observable;

        /// <summary>
        ///     Initializes a new instance of the <see cref="RealtimeDatabase{T}" /> class.
        /// </summary>
        /// <param name="childQuery"> The child query.  </param>
        /// <param name="elementRoot"> The element Root. </param>
        /// <param name="offlineDatabaseFactory"> The offline database factory.  </param>
        /// <param name="filenameModifier"> Custom string which will get appended to the file name.  </param>
        /// <param name="streamChanges"> Specifies whether changes should be streamed from the server.  </param>
        /// <param name="pullEverythingOnStart">
        ///     Specifies if everything should be pull from the online storage on start. It only
        ///     makes sense when <see cref="streamChanges" /> is set to true.
        /// </param>
        /// <param name="pushChanges">
        ///     Specifies whether changed items should actually be pushed to the server. If this is false,
        ///     then Put / Post / Delete will not affect server data.
        /// </param>
        public RealtimeDatabase(ChildQuery childQuery, string elementRoot,
            Func<Type, string, IDictionary<string, OfflineEntry>> offlineDatabaseFactory, string filenameModifier,
            StreamingOptions streamingOptions, InitialPullStrategy initialPullStrategy, bool pushChanges,
            ISetHandler<T> setHandler = null)
        {
            this.childQuery = childQuery;
            this.elementRoot = elementRoot;
            this.streamingOptions = streamingOptions;
            this.initialPullStrategy = initialPullStrategy;
            this.pushChanges = pushChanges;
            Database = offlineDatabaseFactory(typeof(T), filenameModifier);
            firebaseCache = new FirebaseCache<T>(new OfflineCacheAdapter<string, T>(Database));
            subject = new Subject<FirebaseEvent<T>>();

            PutHandler = setHandler ?? new SetHandler<T>();

            isSyncRunning = true;
            Task.Factory.StartNew(SynchronizeThread, CancellationToken.None, TaskCreationOptions.LongRunning,
                TaskScheduler.Default);
        }

        /// <summary>
        ///     Gets the backing Database.
        /// </summary>
        public IDictionary<string, OfflineEntry> Database { get; }

        public ISetHandler<T> PutHandler { private get; set; }

        public void Dispose()
        {
            subject.OnCompleted();
            firebaseSubscription?.Dispose();
        }

        /// <summary>
        ///     Event raised whenever an exception is thrown in the synchronization thread. Exception thrown in there are
        ///     swallowed, so this event is the only way to get to them.
        /// </summary>
        public event EventHandler<ExceptionEventArgs> SyncExceptionThrown;

        /// <summary>
        ///     Overwrites existing object with given key.
        /// </summary>
        /// <param name="key"> The key. </param>
        /// <param name="obj"> The object to set. </param>
        /// <param name="syncOnline"> Indicates whether the item should be synced online. </param>
        /// <param name="priority">
        ///     The priority. Objects with higher priority will be synced first. Higher number indicates higher
        ///     priority.
        /// </param>
        public void Set(string key, T obj, SyncOptions syncOptions, int priority = 1)
        {
            SetAndRaise(key, new OfflineEntry(key, obj, priority, syncOptions));
        }

        public void Set<TProperty>(string key, Expression<Func<T, TProperty>> propertyExpression, object value,
            SyncOptions syncOptions, int priority = 1)
        {
            var fullKey = GenerateFullKey(key, propertyExpression, syncOptions);
            var serializedObject = JsonConvert.SerializeObject(value).Trim('"', '\\');

            if (fullKey.Item3)
            {
                if (typeof(TProperty) != typeof(string) || value == null)
                    // don't escape non-string primitives and null;
                    serializedObject = $"{{ \"{fullKey.Item2}\" : {serializedObject} }}";
                else
                    serializedObject = $"{{ \"{fullKey.Item2}\" : \"{serializedObject}\" }}";
            }

            var setObject = firebaseCache.PushData("/" + fullKey.Item1, serializedObject).First();

            if (!Database.ContainsKey(key) || Database[key].SyncOptions != SyncOptions.Patch &&
                Database[key].SyncOptions != SyncOptions.Put)
                Database[fullKey.Item1] =
                    new OfflineEntry(fullKey.Item1, value, serializedObject, priority, syncOptions, true);

            subject.OnNext(new FirebaseEvent<T>(key, setObject.Object,
                setObject == null ? FirebaseEventType.Delete : FirebaseEventType.InsertOrUpdate,
                FirebaseEventSource.Offline));
        }

        /// <summary>
        ///     Fetches an object with the given key and adds it to the Database.
        /// </summary>
        /// <param name="key"> The key. </param>
        /// <param name="priority">
        ///     The priority. Objects with higher priority will be synced first. Higher number indicates higher
        ///     priority.
        /// </param>
        public void Pull(string key, int priority = 1)
        {
            if (!Database.ContainsKey(key))
                Database[key] = new OfflineEntry(key, null, priority, SyncOptions.Pull);
            else if (Database[key].SyncOptions == SyncOptions.None)
                // pull only if push isn't pending
                Database[key].SyncOptions = SyncOptions.Pull;
        }

        /// <summary>
        ///     Fetches everything from the remote database.
        /// </summary>
        public async Task PullAsync()
        {
            var existingEntries = await childQuery
                .OnceAsync<T>()
                .ToObservable()
                .RetryAfterDelay<IReadOnlyCollection<FirebaseObject<T>>, FirebaseException>(
                    childQuery.Client.Options.SyncPeriod,
                    ex => ex.StatusCode ==
                          HttpStatusCode
                              .OK) // OK implies the request couldn't complete due to network error. 
                .Select(e => ResetDatabaseFromInitial(e, false))
                .SelectMany(e => e)
                .Do(e =>
                {
                    Database[e.Key] = new OfflineEntry(e.Key, e.Object, 1, SyncOptions.None);
                    subject.OnNext(new FirebaseEvent<T>(e.Key, e.Object, FirebaseEventType.InsertOrUpdate,
                        FirebaseEventSource.OnlinePull));
                })
                .ToList();

            // Remove items not stored online
            foreach (var item in Database.Keys.Except(existingEntries.Select(f => f.Key)).ToList())
            {
                Database.Remove(item);
                subject.OnNext(new FirebaseEvent<T>(item, null, FirebaseEventType.Delete,
                    FirebaseEventSource.OnlinePull));
            }
        }

        /// <summary>
        ///     Retrieves all offline items currently stored in local database.
        /// </summary>
        public IEnumerable<FirebaseObject<T>> Once()
        {
            return Database
                .Where(kvp => !string.IsNullOrEmpty(kvp.Value.Data) && kvp.Value.Data != "null" && !kvp.Value.IsPartial)
                .Select(kvp => new FirebaseObject<T>(kvp.Key, kvp.Value.Deserialize<T>()))
                .ToList();
        }

        /// <summary>
        ///     Starts observing the real-time Database. Events will be fired both when change is done locally and remotely.
        /// </summary>
        /// <returns> Stream of <see cref="FirebaseEvent{T}" />. </returns>
        public IObservable<FirebaseEvent<T>> AsObservable()
        {
            if (!isSyncRunning)
            {
                isSyncRunning = true;
                Task.Factory.StartNew(SynchronizeThread, CancellationToken.None, TaskCreationOptions.LongRunning,
                    TaskScheduler.Default);
            }

            if (observable == null)
            {
                var initialData = Observable.Return(FirebaseEvent<T>.Empty(FirebaseEventSource.Offline));
                if (Database.TryGetValue(elementRoot, out var oe))
                    initialData = Observable.Return(oe)
                        .Where(offlineEntry =>
                            !string.IsNullOrEmpty(offlineEntry.Data) && offlineEntry.Data != "null" &&
                            !offlineEntry.IsPartial)
                        .Select(offlineEntry => new FirebaseEvent<T>(offlineEntry.Key, offlineEntry.Deserialize<T>(),
                            FirebaseEventType.InsertOrUpdate, FirebaseEventSource.Offline));
                else if (Database.Count > 0)
                    initialData = Database
                        .Where(kvp =>
                            !string.IsNullOrEmpty(kvp.Value.Data) && kvp.Value.Data != "null" && !kvp.Value.IsPartial)
                        .Select(kvp => new FirebaseEvent<T>(kvp.Key, kvp.Value.Deserialize<T>(),
                            FirebaseEventType.InsertOrUpdate, FirebaseEventSource.Offline))
                        .ToList()
                        .ToObservable();

                observable = initialData
                    .Merge(subject)
                    .Merge(GetInitialPullObservable()
                        .RetryAfterDelay<IReadOnlyCollection<FirebaseObject<T>>, FirebaseException>(
                            childQuery.Client.Options.SyncPeriod,
                            ex => ex.StatusCode ==
                                  HttpStatusCode
                                      .OK) // OK implies the request couldn't complete due to network error. 
                        .Select(e => ResetDatabaseFromInitial(e))
                        .SelectMany(e => e)
                        .Do(SetObjectFromInitialPull)
                        .Select(e => new FirebaseEvent<T>(e.Key, e.Object,
                            e.Object == null ? FirebaseEventType.Delete : FirebaseEventType.InsertOrUpdate,
                            FirebaseEventSource.OnlineInitial))
                        .Concat(Observable.Create<FirebaseEvent<T>>(observer =>
                            InitializeStreamingSubscription(observer))))
                    .Do(next => { }, e => observable = null, () => observable = null)
                    .Replay()
                    .RefCount();
            }

            return observable;
        }

        private IReadOnlyCollection<FirebaseObject<T>> ResetDatabaseFromInitial(
            IReadOnlyCollection<FirebaseObject<T>> collection, bool onlyWhenInitialEverything = true)
        {
            if (onlyWhenInitialEverything && initialPullStrategy != InitialPullStrategy.Everything) return collection;

            // items which are in local db, but not in the online collection
            var extra = Once()
                .Select(f => f.Key)
                .Except(collection.Select(c => c.Key))
                .Select(k => new FirebaseObject<T>(k, null));

            return collection.Concat(extra).ToList();
        }

        private void SetObjectFromInitialPull(FirebaseObject<T> e)
        {
            // set object with no sync only if it doesn't exist yet 
            // and the InitialPullStrategy != Everything
            // this attempts to deal with scenario when you are offline, have local changes and go online
            // in this case having the InitialPullStrategy set to everything would basically purge all local changes
            if (!Database.ContainsKey(e.Key) || Database[e.Key].SyncOptions == SyncOptions.None ||
                Database[e.Key].SyncOptions == SyncOptions.Pull ||
                initialPullStrategy != InitialPullStrategy.Everything)
                Database[e.Key] = new OfflineEntry(e.Key, e.Object, 1, SyncOptions.None);
        }

        private IObservable<IReadOnlyCollection<FirebaseObject<T>>> GetInitialPullObservable()
        {
            FirebaseQuery query;
            switch (initialPullStrategy)
            {
                case InitialPullStrategy.MissingOnly:
                    query = childQuery.OrderByKey().StartAt(() => GetLatestKey());
                    break;
                case InitialPullStrategy.Everything:
                    query = childQuery;
                    break;
                case InitialPullStrategy.None:
                default:
                    return Observable.Empty<IReadOnlyCollection<FirebaseEvent<T>>>();
            }

            if (string.IsNullOrWhiteSpace(elementRoot))
                return Observable.Defer(() => query.OnceAsync<T>().ToObservable());

            // there is an element root, which indicates the target location is not a collection but a single element
            return Observable.Defer(async () =>
                Observable.Return(await query.OnceSingleAsync<T>())
                    .Select(e => new[] {new FirebaseObject<T>(elementRoot, e)}));
        }

        private IDisposable InitializeStreamingSubscription(IObserver<FirebaseEvent<T>> observer)
        {
            var completeDisposable = Disposable.Create(() => isSyncRunning = false);

            switch (streamingOptions)
            {
                case StreamingOptions.LatestOnly:
                    // stream since the latest key
                    var queryLatest = childQuery.OrderByKey().StartAt(() => GetLatestKey());
                    firebaseSubscription =
                        new FirebaseSubscription<T>(observer, queryLatest, elementRoot, firebaseCache);
                    firebaseSubscription.ExceptionThrown += StreamingExceptionThrown;

                    return new CompositeDisposable(firebaseSubscription.Run(), completeDisposable);
                case StreamingOptions.Everything:
                    // stream everything
                    var queryAll = childQuery;
                    firebaseSubscription = new FirebaseSubscription<T>(observer, queryAll, elementRoot, firebaseCache);
                    firebaseSubscription.ExceptionThrown += StreamingExceptionThrown;

                    return new CompositeDisposable(firebaseSubscription.Run(), completeDisposable);
            }

            return completeDisposable;
        }

        private void SetAndRaise(string key, OfflineEntry obj,
            FirebaseEventSource eventSource = FirebaseEventSource.Offline)
        {
            Database[key] = obj;
            subject.OnNext(new FirebaseEvent<T>(key, obj?.Deserialize<T>(),
                string.IsNullOrEmpty(obj?.Data) || obj?.Data == "null"
                    ? FirebaseEventType.Delete
                    : FirebaseEventType.InsertOrUpdate, eventSource));
        }

        private async void SynchronizeThread()
        {
            while (isSyncRunning)
            {
                try
                {
                    var validEntries = Database.Where(e => e.Value != null);
                    await PullEntriesAsync(validEntries.Where(kvp => kvp.Value.SyncOptions == SyncOptions.Pull));

                    if (pushChanges)
                        await PushEntriesAsync(validEntries.Where(kvp =>
                            kvp.Value.SyncOptions == SyncOptions.Put || kvp.Value.SyncOptions == SyncOptions.Patch));
                }
                catch (Exception ex)
                {
                    SyncExceptionThrown?.Invoke(this, new ExceptionEventArgs(ex));
                }

                await Task.Delay(childQuery.Client.Options.SyncPeriod);
            }
        }

        private string GetLatestKey()
        {
            var key = Database.OrderBy(o => o.Key, StringComparer.Ordinal).LastOrDefault().Key ?? string.Empty;

            if (!string.IsNullOrWhiteSpace(key))
                key = key.Substring(0, key.Length - 1) + (char) (key[key.Length - 1] + 1);

            return key;
        }

        private async Task PushEntriesAsync(IEnumerable<KeyValuePair<string, OfflineEntry>> pushEntries)
        {
            var groups = pushEntries.GroupBy(pair => pair.Value.Priority).OrderByDescending(kvp => kvp.Key).ToList();

            foreach (var group in groups)
            {
                var tasks = group.OrderBy(kvp => kvp.Value.IsPartial).Select(kvp =>
                    kvp.Value.IsPartial
                        ? ResetSyncAfterPush(PutHandler.SetAsync(childQuery, kvp.Key, kvp.Value), kvp.Key)
                        : ResetSyncAfterPush(PutHandler.SetAsync(childQuery, kvp.Key, kvp.Value), kvp.Key,
                            kvp.Value.Deserialize<T>()));

                try
                {
                    await Task.WhenAll(tasks).WithAggregateException();
                }
                catch (Exception ex)
                {
                    SyncExceptionThrown?.Invoke(this, new ExceptionEventArgs(ex));
                }
            }
        }

        private async Task PullEntriesAsync(IEnumerable<KeyValuePair<string, OfflineEntry>> pullEntries)
        {
            var taskGroups = pullEntries.GroupBy(pair => pair.Value.Priority).OrderByDescending(kvp => kvp.Key);

            foreach (var group in taskGroups)
            {
                var tasks = group.Select(pair =>
                    ResetAfterPull(
                        childQuery.Child(pair.Key == elementRoot ? string.Empty : pair.Key).OnceSingleAsync<T>(),
                        pair.Key, pair.Value));

                try
                {
                    await Task.WhenAll(tasks).WithAggregateException();
                }
                catch (Exception ex)
                {
                    SyncExceptionThrown?.Invoke(this, new ExceptionEventArgs(ex));
                }
            }
        }

        private async Task ResetAfterPull(Task<T> task, string key, OfflineEntry entry)
        {
            await task;
            SetAndRaise(key, new OfflineEntry(key, task.Result, entry.Priority, SyncOptions.None),
                FirebaseEventSource.OnlinePull);
        }

        private async Task ResetSyncAfterPush(Task task, string key, T obj)
        {
            await ResetSyncAfterPush(task, key);

            if (streamingOptions == StreamingOptions.None)
                subject.OnNext(new FirebaseEvent<T>(key, obj,
                    obj == null ? FirebaseEventType.Delete : FirebaseEventType.InsertOrUpdate,
                    FirebaseEventSource.OnlinePush));
        }

        private async Task ResetSyncAfterPush(Task task, string key)
        {
            await task;
            ResetSyncOptions(key);
        }

        private void ResetSyncOptions(string key)
        {
            var item = Database[key];

            if (item.IsPartial)
            {
                Database.Remove(key);
            }
            else
            {
                item.SyncOptions = SyncOptions.None;
                Database[key] = item;
            }
        }

        private void StreamingExceptionThrown(object sender, ExceptionEventArgs<FirebaseException> e)
        {
            SyncExceptionThrown?.Invoke(this, new ExceptionEventArgs(e.Exception));
        }

        private Tuple<string, string, bool> GenerateFullKey<TProperty>(string key,
            Expression<Func<T, TProperty>> propertyGetter, SyncOptions syncOptions)
        {
            var visitor = new MemberAccessVisitor();
            visitor.Visit(propertyGetter);
            var propertyType = typeof(TProperty).GetTypeInfo();
            var prefix = key == string.Empty ? string.Empty : key + "/";

            // primitive types
            if (syncOptions == SyncOptions.Patch && (propertyType.IsPrimitive ||
                                                     Nullable.GetUnderlyingType(typeof(TProperty)) != null ||
                                                     typeof(TProperty) == typeof(string)))
                return Tuple.Create(prefix + string.Join("/", visitor.PropertyNames.Skip(1).Reverse()),
                    visitor.PropertyNames.First(), true);

            return Tuple.Create(prefix + string.Join("/", visitor.PropertyNames.Reverse()),
                visitor.PropertyNames.First(), false);
        }
    }
}