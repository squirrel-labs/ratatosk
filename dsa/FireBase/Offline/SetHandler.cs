using System.Threading.Tasks;
using Firebase.Database.Query;

namespace Firebase.Database.Offline
{
    public class SetHandler<T> : ISetHandler<T>
    {
        public virtual async Task SetAsync(ChildQuery query, string key, OfflineEntry entry)
        {
            using (var child = query.Child(key))
            {
                if (entry.SyncOptions == SyncOptions.Put)
                    await child.PutAsync(entry.Data);
                else
                    await child.PatchAsync(entry.Data);
            }
        }
    }
}