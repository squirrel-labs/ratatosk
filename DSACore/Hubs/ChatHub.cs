using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using DSACore.DSA_Game.Characters;
using DSACore.FireBase;
using DSACore.Models;
using DSACore.Models.Network;
using Microsoft.AspNetCore.SignalR;

namespace DSACore.Hubs
{
    public class ChatHub : Hub
    {
        //private static Dictionary<string, User> UserGroup = new Dictionary<string, User>();

        private static List<Group> DSAGroups = new List<Group>();

        static ChatHub()
        {
            DSAGroups = Database.GetGroups().Result;
            //AddGroups();
        }

        private static async void AddGroups()
        {
            await Database.AddGroup(new Models.Database.Group { Name = "HalloWelt", Password = "valid" });
            await Database.AddGroup(new Models.Database.Group { Name = "Die Krassen Gamer", Password = "valid" });
            await Database.AddGroup(new Models.Database.Group { Name = "DSA", Password = "valid" });
            await Database.AddGroup(new Models.Database.Group { Name = "Die Überhelden", Password = "valid" });
        }

        public async Task SendMessage(string user, string message)
        {
            var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();
            var ident = args.First().Replace("!", ""); 
            if(args.Count>0){args.RemoveAt(0);}

            try
            {
                string group = getGroup(Context.ConnectionId).Name;
                await SendToGroup(Commands.CommandHandler.ExecuteCommand(new Command { CharId = 0, CmdIdentifier = ident, CmdTexts = args, Name = user }));
            }
            catch(InvalidOperationException e)
            {
                await Clients.Caller.SendCoreAsync("ReceiveMessage", new object[] {"Nutzer ist in keiner Gruppe. Erst joinen!"});
            }
            
        }

        private Task SendToGroup(string message)
        {
            string group = getGroup(Context.ConnectionId).Name;
            return Clients.Group(group).SendCoreAsync("ReceiveMessage", new object[] { getUser(Context.ConnectionId).Name, message });
        }

        private Models.Network.Group getGroup(string id)
        {
            return DSAGroups.First(x => x.Users.Exists(y => y.ConnectionId.Equals(id)));
        }

        private User getUser(string id)
        {
            return DSAGroups.First(x => x.Users.Exists(y => y.ConnectionId.Equals(id))).Users.First(z => z.ConnectionId.Equals(id));
        }

        public async Task GetGroups()
        {
            var test = Database.GetGroups();
            test.Wait();
            foreach (var group in test.Result)
            {
                if (!DSAGroups.Exists(x => x.Name.Equals(group.Name)))
                {
                    DSAGroups.Add(group);
                }
            }

            await  Clients.Caller.SendCoreAsync("ListGroups", new object[] { DSAGroups.Select(x=>x.SendGroup()) });
            //throw new NotImplementedException("add database call to get groups");
        }

        public async Task AddGroup(string group, string password)
        {
            DSAGroups.Add(new Group(group, password));
            var Dgroup = new DSACore.Models.Database.Group{Name = group, Id = DSAGroups.Count-1};
            //Database.AddGroup(Dgroup);
            await Clients.Caller.SendCoreAsync("ReceiveMessage", new[] {$"group {@group} sucessfully added"});
            //throw new NotImplementedException("add database call to add groups");
        }

        public async Task UploadChar(string xml)
        {
            var group = getGroup(Context.ConnectionId);

            Database.AddChar(new Character(new MemoryStream(Encoding.UTF8.GetBytes(xml))), group);
            //throw new NotImplementedException("add database call to add groups");
        }

        public async Task Login(string group, string user, string hash)
        {
            //string password = System.Text.Encoding.UTF8.GetString(hash);
            if (hash == DSAGroups.First(x=>x.Name == group).Password)
            {
                DSAGroups.First(x=>x.Name.Equals(group)).Users.Add(new User{ConnectionId = Context.ConnectionId, Name = user});
                await Groups.AddToGroupAsync(Context.ConnectionId, group);

                await SendToGroup("Ein neuer Nutzer hat die Gruppe betreten");
            }
            else
            {

                await Clients.Caller.SendAsync("ReceiveMessage", "Falsches Passwort!");
            }
        }

        public override Task OnDisconnectedAsync(Exception exception)
        {
            Disconnect().Wait();
            return base.OnDisconnectedAsync(exception);
        }

        public async Task Disconnect()
        {
            try
            {
                var group = getGroup(Context.ConnectionId);


                var user = getUser(Context.ConnectionId);
                await SendToGroup(user.Name + " disconnected from the Server");
                group.Users.Remove(user);
                await Groups.RemoveFromGroupAsync(Context.ConnectionId, group.Name);
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
                //throw;
            }

        }

    }
}
