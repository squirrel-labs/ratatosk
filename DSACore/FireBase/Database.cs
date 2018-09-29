using System;
using System.Collections.Generic;
using System.IO;
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
            var auth = File.ReadAllText("Token"); ; // your app secret
            var firebaseClient = new FirebaseClient(
                "https://heldenonline-4d828.firebaseio.com/",
                new FirebaseOptions
                {
                    AuthTokenAsyncFactory = () => Task.FromResult(auth)
                });
        }

        public static void DoStuff(){}
    }
}
