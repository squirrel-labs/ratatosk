using System.Threading.Tasks;
using Firebase.Database.Query;

namespace Firebase.Database.Offline
{
    public interface ISetHandler<in T>
    {
        Task SetAsync(ChildQuery query, string key, OfflineEntry entry);
    }
}