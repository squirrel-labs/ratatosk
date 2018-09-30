using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using DSACore.DSA_Game.Characters;
using DSACore.Models.Database;
using DSACore.Models;
using Firebase.Database;
using Firebase.Database.Query;


namespace DSACore.FireBase
{
    public static class Database
    {
        public static FirebaseClient firebase;

        static Database()
        {
            var auth = File.ReadAllText(DSACore.DSA_Game.Dsa.rootPath+"Token"); ; // your app secret
            firebase = new FirebaseClient(
                "https://heldenonline-4d828.firebaseio.com/",
                new FirebaseOptions
                {
                    AuthTokenAsyncFactory = () => Task.FromResult(auth)
                });
        }
        

        public static async Task<int> AddChar(Character file, Models.Network.Group group)
        {
            DatabaseChar.LoadChar(file, out GroupChar groupChar, out DatabaseChar data);
            
            var lastChar = await firebase
                .Child("Chars")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<DatabaseChar>();
            int id = groupChar.Id = data.Id = lastChar.First().Object.Id + 1;
            
            await firebase
                .Child("Groups")
                .Child("Char" + id)
                .PutAsync(data);

            await firebase
                .Child("Chars")
                .Child("Char" + id)
                .PutAsync(data);

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

            await firebase
                .Child("Inventories")
                .Child("Inventory" + id)
                .DeleteAsync();
            
        }

        public static async Task<DatabaseChar> GetChar(int id)
        {
            var chr = await firebase
                .Child("Chars")
                .Child("Char" + id)
                .OnceSingleAsync<DatabaseChar>();
            return chr;
        }

        public static async Task<Inventory> GetInventory(int id)
        {
            var inv = await firebase
                .Child("Inventories")
                .Child("Inventory"+id)
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
            return await firebase
                .Child("Talents")
                .Child(talent)
                .OnceSingleAsync<Talent>();
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
            return await firebase
                .Child("Spells")
                .Child(spell)
                .OnceSingleAsync<GeneralSpell>();
        }
        

        public static async Task AddWeapon(Weapon wep)
        {
            await firebase
                .Child("Weapons")
                .Child(wep.Name)
                .PutAsync(wep);
        }

        public static async Task RemoveWeapon(string weapon)
        {
            await firebase
                .Child("Weapons")
                .Child(weapon)
                .DeleteAsync();
        }

        public static async Task< Weapon> GetWeapon(string weapon)
        {
            return await firebase
                .Child("Weapons")
                .Child(weapon)
                .OnceSingleAsync<Weapon>();
        }

        public static async Task<List<Models.Network.Group>> GetGroups()
        {
            var groups = await firebase
                .Child("Groups")
                .OrderByKey()
                .OnceAsync<Group>();
            var ret = new List<Models.Network.Group>();

            foreach (var firebaseObject in groups)
            {
                ret.Add(new Models.Network.Group(firebaseObject.Object.Name, firebaseObject.Object.Password));
            }

            return ret;
        }

        public static async Task<Group> GetGroup(int id)
        {
            var group = await firebase
                .Child("Groups")
                .Child("Group"+id)
                .OnceSingleAsync<Group>();
            return group;
        }

        public static async Task AddGroup(Group group)
        {
            var lastChar = await firebase
                .Child("Groups")
                .OrderByKey()
                .LimitToLast(1)
                .OnceAsync<Group>();
            int id = group.Id = lastChar.First().Object.Id + 1;

            await firebase
                .Child("Groups")
                .Child("Group"+id)
                .PutAsync(group);
        }

        public static async void SetGroup(Group group)
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
