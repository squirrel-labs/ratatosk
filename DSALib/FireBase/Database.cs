using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using DSALib.DSA_Game;
using DSALib.DSA_Game.Characters;
using DSALib.Models.Database.Dsa;
using Firebase.Database;
using Firebase.Database.Query;

namespace DSALib.FireBase
{
    public static class Database
    {
        public static FirebaseClient Firebase;

        public static Dictionary<string, DatabaseChar> Chars = new Dictionary<string, DatabaseChar>();

        public static Dictionary<string, MeleeWeapon> MeleeList = new Dictionary<string, MeleeWeapon>();

        public static Dictionary<string, RangedWeapon> RangedWeapons = new Dictionary<string, RangedWeapon>();

        public static Dictionary<string, DSALib.Models.Database.Dsa.Talent> Talents = new Dictionary<string, DSALib.Models.Database.Dsa.Talent>();

        public static Dictionary<string, GeneralSpell> Spells = new Dictionary<string, GeneralSpell>();

        static Database()
        {
            var auth = File.ReadAllText(Dsa.rootPath + "Token"); // your app secret
            Firebase = new FirebaseClient(
                "https://heldenonline-4d828.firebaseio.com/",
                new FirebaseOptions
                {
                    AuthTokenAsyncFactory = () => Task.FromResult(auth)
                });

            Task.Run(Initialize);
        }

        private static  void Initialize() {
            var waiting = new[] {
                // ToDo IntializeCollection("Chars", Chars),
                IntializeCollection("MeleeWeapons", MeleeList),
                IntializeCollection("RangedWeapons", RangedWeapons),
                IntializeCollection("Talents", Talents),
                IntializeCollection("Spells", Spells),
            };
            Task.WaitAll(waiting);
        }

        private static async Task IntializeCollection<T>(string path, Dictionary<string, T> list)
        {
            var temp = await Firebase
                .Child(path)
                .OrderByKey()
                .OnceAsync<T>();

            foreach (var firebaseObject in temp) list.Add(firebaseObject.Key, firebaseObject.Object);
        }

        public static async Task<int> AddChar(Character file, string group)
        {
            DatabaseChar.LoadChar(file, out var groupChar, out var data);

            var lastChar = await Firebase
                .Child("Chars")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<DatabaseChar>();
            var id = groupChar.Id = data.Id = lastChar.First().Object.Id + 1;

            await Firebase //TODO Reomve await Operators
                .Child("Groups")
                .Child("Char" + id)
                .PutAsync(groupChar);

            await Firebase
                .Child("Chars")
                .Child("Char" + id)
                .PutAsync(data);

            Chars["Char" + id] = data;

            await Firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .PutAsync(new Inventory());

            return id + 1;
        }

        public static async Task RemoveChar(int id)
        {
            await Firebase
                .Child("Groups")
                .Child("Char" + id)
                .DeleteAsync();

            await Firebase
                .Child("Chars")
                .Child("Char" + id)
                .DeleteAsync();

            Chars.Remove("Char" + id);

            await Firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .DeleteAsync();
        }

        public static DatabaseChar GetChar(int id)
        {
            /*var chr = await firebase
                .Child("Chars")
                .Child("Char" + id)
                .OnceSingleAsync<DatabaseChar>();
            return chr;*/
            return Chars["Char" + id];
        }

        public static async Task<Inventory> GetInventory(int id)
        {
            var inv = await Firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .OnceSingleAsync<Inventory>();
            return inv;
        }

        public static async Task SetInventory(int id, Inventory inv)
        {
            await Firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .PutAsync(inv);
        }

        public static async Task AddTalent(DSALib.Models.Database.Dsa.Talent tal)
        {
            await Firebase
                .Child("Talents")
                .Child(tal.Name)
                .PutAsync(tal);
        }

        public static async Task RemoveTalent(string talent)
        {
            await Firebase
                .Child("Talents")
                .Child(talent)
                .DeleteAsync();
        }

        public static DSALib.Models.Database.Dsa.Talent GetTalent(string talent)
        {
            /*
                        return await firebase
                            .Child("Talents")
                            .Child(talent)
                            .OnceSingleAsync<Talent>();*/
            return Talents[talent];
        }

        public static async Task AddSpell(GeneralSpell tal)
        {
            await Firebase
                .Child("Spells")
                .Child(tal.Name)
                .PutAsync(tal);
        }

        public static async Task RemoveSpell(string spell)
        {
            await Firebase
                .Child("Spells")
                .Child(spell)
                .DeleteAsync();
        }

        public static GeneralSpell GetSpell(string spell)
        {
            return Spells[spell];
        }


        public static async Task AddWeapon(Weapon wep)
        {
            var collection = wep.GetType() == typeof(MeleeWeapon) ? "MeleeWeapons" : "RangedWeapons";
            await Firebase
                .Child(collection)
                .Child(wep.Name)
                .PutAsync(wep);
        }

        public static async Task RemoveWeapon(string weapon, bool ranged = false)
        {
            var collection = ranged ? "RangedWeapons" : "MeleeWeapons";
            await Firebase
                .Child(collection)
                .Child(weapon)
                .DeleteAsync();
        }

        public static async Task<Weapon> GetWeapon(string weapon, bool ranged = false)
        {
            var collection = ranged ? "RangedWeapons" : "MeleeWeapons";
            return await Firebase
                .Child(collection)
                .Child(weapon)
                .OnceSingleAsync<Weapon>();
        }

        public static async Task<List<Tuple<string, string>>> GetGroups()
        {
            var groups = await Firebase
                .Child("Groups")
                .OrderByKey()
                .OnceAsync<DSALib.Models.Database.Groups.Group>();
            var ret = new List<Tuple<string, string>>();

            foreach (var firebaseObject in groups)
                ret.Add(new Tuple<string, string>(firebaseObject.Object.Name, firebaseObject.Object.Password));

            return ret;
        }

        public static async Task<DSALib.Models.Database.Groups.Group> GetGroup(int id)
        {
            var group = await Firebase
                .Child("Groups")
                .Child("Group" + id)
                .OnceSingleAsync<DSALib.Models.Database.Groups.Group>();
            return group;
        }

        public static async Task AddGroup(DSALib.Models.Database.Groups.Group group)
        {
            var lastChar = await Firebase
                .Child("Groups")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<DSALib.Models.Database.Groups.Group>();
            var id = group.Id = lastChar.First().Object.Id + 1;

            await Firebase
                .Child("Groups")
                .Child("Group" + id)
                .PutAsync(group);
        }

        public static async void SetGroup(DSALib.Models.Database.Groups.Group group)
        {
            await Firebase
                .Child("Groups")
                .Child("Group" + group.Id)
                .PutAsync(group);
        }

        public static async void DeleteGroup(int id)
        {
            await Firebase
                .Child("Groups")
                .Child("Group" + id)
                .DeleteAsync();
        }
    }
}