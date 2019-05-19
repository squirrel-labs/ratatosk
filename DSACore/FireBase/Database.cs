using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DSACore.DSA_Game;
using DSACore.DSA_Game.Characters;
using DSACore.Models.Database.DSA;
using DSACore.Models.Network;
using Firebase.Database;
using Firebase.Database.Query;

namespace DSACore.FireBase
{
    public static class Database
    {
        public static FirebaseClient firebase;

        public static Dictionary<string, DatabaseChar> Chars = new Dictionary<string, DatabaseChar>();

        public static Dictionary<string, MeleeWeapon> MeleeList = new Dictionary<string, MeleeWeapon>();

        public static Dictionary<string, RangedWeapon> RangedWeapons = new Dictionary<string, RangedWeapon>();

        public static Dictionary<string, Talent> Talents = new Dictionary<string, Talent>();

        public static Dictionary<string, GeneralSpell> Spells = new Dictionary<string, GeneralSpell>();

        static Database()
        {
            var auth = File.ReadAllText(Dsa.rootPath + "Token");
            ; // your app secret
            firebase = new FirebaseClient(
                "https://heldenonline-4d828.firebaseio.com/",
                new FirebaseOptions
                {
                    AuthTokenAsyncFactory = () => Task.FromResult(auth)
                });

            Initialize();
        }

        private static async Task Initialize()
        {
            IntializeCollection("Chars", Chars);
            IntializeCollection("MeleeWeapons", MeleeList);
            IntializeCollection("RangedWeapons", RangedWeapons);
            IntializeCollection("Talents", Talents);
            IntializeCollection("Spells", Spells);
        }

        private static async Task IntializeCollection<T>(string path, Dictionary<string, T> list)
        {
            var temp = await firebase
                .Child(path)
                .OrderByKey()
                .OnceAsync<T>();

            foreach (var firebaseObject in temp) list.Add(firebaseObject.Key, firebaseObject.Object);
        }

        public static async Task<int> AddChar(Character file, Group group)
        {
            DatabaseChar.LoadChar(file, out var groupChar, out var data);

            var lastChar = await firebase
                .Child("Chars")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<DatabaseChar>();
            var id = groupChar.Id = data.Id = lastChar.First().Object.Id + 1;

            await firebase //TODO Reomve await Operators
                .Child("Groups")
                .Child("Char" + id)
                .PutAsync(groupChar);

            await firebase
                .Child("Chars")
                .Child("Char" + id)
                .PutAsync(data);

            Chars["Char" + id] = data;

            await firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .PutAsync(new Inventory());

            return id + 1;
        }

        public static async Task RemoveChar(int id)
        {
            await firebase
                .Child("Groups")
                .Child("Char" + id)
                .DeleteAsync();

            await firebase
                .Child("Chars")
                .Child("Char" + id)
                .DeleteAsync();

            Chars.Remove("Char" + id);

            await firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .DeleteAsync();
        }

        public static async Task<DatabaseChar> GetChar(int id)
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
            var inv = await firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .OnceSingleAsync<Inventory>();
            return inv;
        }

        public static async Task SetInventory(int id, Inventory inv)
        {
            await firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .PutAsync(inv);
        }

        public static async Task AddTalent(Talent tal)
        {
            await firebase
                .Child("Talents")
                .Child(tal.Name)
                .PutAsync(tal);
        }

        public static async Task RemoveTalent(string talent)
        {
            await firebase
                .Child("Talents")
                .Child(talent)
                .DeleteAsync();
        }

        public static async Task<Talent> GetTalent(string talent)
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
            await firebase
                .Child("Spells")
                .Child(tal.Name)
                .PutAsync(tal);
        }

        public static async Task RemoveSpell(string spell)
        {
            await firebase
                .Child("Spells")
                .Child(spell)
                .DeleteAsync();
        }

        public static async Task<GeneralSpell> GetSpell(string spell)
        {
            /*return await firebase
                .Child("Spells")
                .Child(spell)
                .OnceSingleAsync<GeneralSpell>();*/
            return Spells[spell];
        }


        public static async Task AddWeapon(Weapon wep)
        {
            var collection = wep.GetType() == typeof(MeleeWeapon) ? "MeleeWeapons" : "RangedWeapons";
            await firebase
                .Child(collection)
                .Child(wep.Name)
                .PutAsync(wep);
        }

        public static async Task RemoveWeapon(string weapon, bool ranged = false)
        {
            var collection = ranged ? "RangedWeapons" : "MeleeWeapons";
            await firebase
                .Child(collection)
                .Child(weapon)
                .DeleteAsync();
        }

        public static async Task<Weapon> GetWeapon(string weapon, bool ranged = false)
        {
            var collection = ranged ? "RangedWeapons" : "MeleeWeapons";
            return await firebase
                .Child(collection)
                .Child(weapon)
                .OnceSingleAsync<Weapon>();
        }

        public static async Task<List<Group>> GetGroups()
        {
            var groups = await firebase
                .Child("Groups")
                .OrderByKey()
                .OnceAsync<Models.Database.Groups.Group>();
            var ret = new List<Group>();

            foreach (var firebaseObject in groups)
                ret.Add(new Group(firebaseObject.Object.Name, firebaseObject.Object.Password));

            return ret;
        }

        public static async Task<Models.Database.Groups.Group> GetGroup(int id)
        {
            var group = await firebase
                .Child("Groups")
                .Child("Group" + id)
                .OnceSingleAsync<Models.Database.Groups.Group>();
            return group;
        }

        public static async Task AddGroup(Models.Database.Groups.Group group)
        {
            var lastChar = await firebase
                .Child("Groups")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<Models.Database.Groups.Group>();
            var id = group.Id = lastChar.First().Object.Id + 1;

            await firebase
                .Child("Groups")
                .Child("Group" + id)
                .PutAsync(group);
        }

        public static async void SetGroup(Models.Database.Groups.Group group)
        {
            await firebase
                .Child("Groups")
                .Child("Group" + group.Id)
                .PutAsync(group);
        }

        public static async void DeleteGroup(int id)
        {
            await firebase
                .Child("Groups")
                .Child("Group" + id)
                .DeleteAsync();
        }
    }
}