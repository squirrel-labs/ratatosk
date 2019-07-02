using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace Firebase.Database.Offline {
    internal class OfflineCacheAdapter<TKey, T> : IDictionary<string, T>, IDictionary {
        private readonly IDictionary<string, OfflineEntry> database;

        public OfflineCacheAdapter(IDictionary<string, OfflineEntry> database) {
            this.database = database;
        }

        public void CopyTo(Array array, int index) {
            throw new NotImplementedException();
        }

        public bool IsSynchronized { get; }

        public object SyncRoot { get; }

        object IDictionary.this[object key] {
            get => database[key.ToString()].Deserialize<T>();

            set {
                var keyString = key.ToString();
                if (database.ContainsKey(keyString))
                    database[keyString] = new OfflineEntry(keyString, value, database[keyString].Priority,
                        database[keyString].SyncOptions);
                else
                    database[keyString] = new OfflineEntry(keyString, value, 1, SyncOptions.None);
            }
        }

        ICollection IDictionary.Values { get; }

        ICollection IDictionary.Keys { get; }

        public bool Contains(object key) {
            return ContainsKey(key.ToString());
        }

        IDictionaryEnumerator IDictionary.GetEnumerator() {
            throw new NotImplementedException();
        }

        public void Remove(object key) {
            Remove(key.ToString());
        }

        public bool IsFixedSize => false;

        public void Add(object key, object value) {
            Add(key.ToString(), (T) value);
        }

        public int Count => database.Count;

        public bool IsReadOnly => database.IsReadOnly;

        public ICollection<string> Keys => database.Keys;

        public ICollection<T> Values => database.Values.Select(o => o.Deserialize<T>()).ToList();

        public T this[string key] {
            get => database[key].Deserialize<T>();

            set {
                if (database.ContainsKey(key))
                    database[key] = new OfflineEntry(key, value, database[key].Priority, database[key].SyncOptions);
                else
                    database[key] = new OfflineEntry(key, value, 1, SyncOptions.None);
            }
        }

        public IEnumerator<KeyValuePair<string, T>> GetEnumerator() {
            return database.Select(d => new KeyValuePair<string, T>(d.Key, d.Value.Deserialize<T>())).GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }

        public void Add(KeyValuePair<string, T> item) {
            Add(item.Key, item.Value);
        }

        public void Clear() {
            database.Clear();
        }

        public bool Contains(KeyValuePair<string, T> item) {
            return ContainsKey(item.Key);
        }

        public void CopyTo(KeyValuePair<string, T>[] array, int arrayIndex) {
            throw new NotImplementedException();
        }

        public bool Remove(KeyValuePair<string, T> item) {
            return database.Remove(item.Key);
        }

        public void Add(string key, T value) {
            database.Add(key, new OfflineEntry(key, value, 1, SyncOptions.None));
        }

        public bool ContainsKey(string key) {
            return database.ContainsKey(key);
        }

        public bool Remove(string key) {
            return database.Remove(key);
        }

        public bool TryGetValue(string key, out T value) {
            OfflineEntry val;

            if (database.TryGetValue(key, out val)) {
                value = val.Deserialize<T>();
                return true;
            }

            value = default(T);
            return false;
        }
    }
}