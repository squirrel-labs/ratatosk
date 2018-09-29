using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Firebase.Database;
using Firebase.Database.Query;


namespace DSACore.FireBase
{
    public static class Database
    {
        static Database()
        {
            var auth = "ABCDE"; // your app secret
            var firebaseClient = new FirebaseClient(
                "<URL>",
                new FirebaseOptions
                {
                    AuthTokenAsyncFactory = () => Task.FromResult(auth)
                });
        }
    }
}
