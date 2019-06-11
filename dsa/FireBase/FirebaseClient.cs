using System;
using System.Net.Http;
using System.Runtime.CompilerServices;
using Firebase.Database.Query;

[assembly: InternalsVisibleTo("Firebase.Database.Tests")]

namespace Firebase.Database
{
    /// <summary>
    ///     Firebase client which acts as an entry point to the online database.
    /// </summary>
    public class FirebaseClient : IDisposable
    {
        private readonly string baseUrl;
        internal readonly HttpClient HttpClient;
        internal readonly FirebaseOptions Options;

        /// <summary>
        ///     Initializes a new instance of the <see cref="FirebaseClient" /> class.
        /// </summary>
        /// <param name="baseUrl"> The base url. </param>
        /// <param name="offlineDatabaseFactory"> Offline database. </param>
        public FirebaseClient(string baseUrl, FirebaseOptions options = null)
        {
            HttpClient = new HttpClient();
            Options = options ?? new FirebaseOptions();

            this.baseUrl = baseUrl;

            if (!this.baseUrl.EndsWith("/")) this.baseUrl += "/";
        }

        public void Dispose()
        {
            HttpClient?.Dispose();
        }

        /// <summary>
        ///     Queries for a child of the data root.
        /// </summary>
        /// <param name="resourceName"> Name of the child. </param>
        /// <returns> <see cref="ChildQuery" />. </returns>
        public ChildQuery Child(string resourceName)
        {
            return new ChildQuery(this, () => baseUrl + resourceName);
        }
    }
}