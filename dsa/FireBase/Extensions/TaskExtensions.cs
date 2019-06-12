using System;
using System.Threading.Tasks;

namespace Firebase.Database.Extensions {
    public static class TaskExtensions {
        /// <summary>
        ///     Instead of unwrapping <see cref="AggregateException" /> it throws it as it is.
        /// </summary>
        public static async Task WithAggregateException(this Task source) {
            try {
                await source.ConfigureAwait(false);
            }
            catch (Exception ex) {
                throw source.Exception ?? ex;
            }
        }
    }
}