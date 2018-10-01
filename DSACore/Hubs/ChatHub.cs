using DSACore.DSA_Game.Characters;
using DSACore.FireBase;
using DSACore.Models.Network;
using Microsoft.AspNetCore.SignalR;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.CodeAnalysis.CSharp.Syntax;

namespace DSACore.Hubs
{
    public class ChatHub : Hub
    {
        //private static Dictionary<string, User> UserGroup = new Dictionary<string, User>();

        private static List<Group> DSAGroups = new List<Group>();

        static ChatHub()
        {
            DSAGroups = Database.GetGroups().Result;
            DSAGroups.Add(new Group("login", ""));
            DSAGroups.Add(new Group("online", ""));
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
            try
            {
                string group = getGroup(Context.ConnectionId).Name;
            }
            catch (InvalidOperationException e)
            {
                //await Clients.Caller.SendCoreAsync("RecieveMessage",
                   // new object[] { "Nutzer ist in keiner Gruppe. Erst joinen!" });
            }

            if (message[0] == '/')
            {
                var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();


                var ident = args.First().Replace("/", "");
                if (args.Count > 0)
                {
                    args.RemoveAt(0);
                }

                var ret = Commands.CommandHandler.ExecuteCommand(new Command
                {
                    CharId = 0,
                    CmdIdentifier = ident,
                    CmdTexts = args,
                    Name = user
                });

                switch (ret.ResponseType)
                {
                    case ResponseType.Caller:
                    case ResponseType.Error:
                        await Clients.Caller.SendAsync("RecieveMessage", ret.message);
                        break;
                    case ResponseType.Broadcast:
                        await SendToGroup(ret.message);
                        break;
                }

                
            }
            else
            {
                await SendToGroup(message);
            }

        }

        private Task SendToGroup(string message)
        {
            try
            {
                string group = getGroup(Context.ConnectionId).Name;
                return Clients.Group(group).SendCoreAsync("RecieveMessage",
                    new object[] {getUser(Context.ConnectionId).Name, message});
            }
            catch (InvalidOperationException e)
            {
                return Clients.Caller.SendCoreAsync("RecieveMessage",
                    new object[] { "Nutzer ist in keiner Gruppe. Erst joinen!" });
            }
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

            await Clients.Caller.SendCoreAsync("ListGroups", new object[] { DSAGroups.Select(x => x.SendGroup()) });
            //throw new NotImplementedException("add database call to get groups");
        }

        public async Task AddGroup(string group, string password)
        {
            DSAGroups.Add(new Group(group, password));
            var Dgroup = new DSACore.Models.Database.Group { Name = group, Id = DSAGroups.Count - 1 };
            //Database.AddGroup(Dgroup);
            await Clients.Caller.SendCoreAsync("RecieveMessage", new[] { $"group {@group} sucessfully added" });
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
            if (hash == DSAGroups.First(x => x.Name == group).Password)
            {
                var gGroup = DSAGroups.First(x => x.Name.Equals(group));
                if (!gGroup.Users.Exists(x => x.Name.Equals(user)))
                {
                    await Groups.RemoveFromGroupAsync(Context.ConnectionId, "login");
                    await Groups.AddToGroupAsync(Context.ConnectionId, group);
                    gGroup.Users.Add(new User { ConnectionId = Context.ConnectionId, Name = user });
                    await SendToGroup("Ein neuer Nutzer hat die Gruppe betreten");
                    await Clients.Caller.SendAsync("LoginResponse", 0);
                    await Clients.Caller.SendAsync("PlayerStatusChanged", new[] {user, "online"});
                }
                else
                {
                    await Clients.Caller.SendAsync("LoginResponse", 1);
                }
            }
            else
            {
                await Clients.Caller.SendAsync("LoginResponse", 2);
                //await Clients.Caller.SendAsync("RecieveMessage", "Falsches Passwort!");
            }
        }

        public override Task OnDisconnectedAsync(Exception exception)
        {
            Disconnect().Wait();
            return base.OnDisconnectedAsync(exception);
        }

        public override Task OnConnectedAsync()
        {
            Groups.AddToGroupAsync(Context.ConnectionId, "login").Wait();
            Groups.AddToGroupAsync(Context.ConnectionId, "online").Wait();
            return base.OnConnectedAsync();
        }

        public async Task Disconnect()
        {
            await Groups.RemoveFromGroupAsync(Context.ConnectionId, "online");
            if (DSAGroups.Exists(x => x.Users.Exists(y => y.ConnectionId == Context.ConnectionId)))
            {
                try
                {
                    var group = getGroup(Context.ConnectionId);


                    var user = getUser(Context.ConnectionId);

                    await Clients.Caller.SendAsync("PlayerStatusChanged", new[] { user.Name, "offline" });
                    //await SendToGroup(user.Name + " disconnected from the Server");
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
}
