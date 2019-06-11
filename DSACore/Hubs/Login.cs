using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using DSACore.Models.Network;
using DSALib.Commands;
using DSALib.DSA_Game.Characters;
using DSALib.FireBase;
using DSALib.Models.Network;
using Microsoft.AspNetCore.SignalR;
using Group = DSACore.Models.Network.Group;

namespace DSACore.Hubs
{
    public class Users : Hub
    {
        //private static Dictionary<string, User> UserGroup = new Dictionary<string, User>();

        private const string ReceiveMethod = "ReceiveMessage"; //receiveMethod;

        static Users() {
            DsaGroups = Database.GetGroups().Result.Select(x=>new Group(x.Item1, x.Item2)).ToList();
            DsaGroups.Add(new Group("login", ""));
            DsaGroups.Add(new Group("online", ""));
            //AddGroups();
        }

        private static List<Group> DsaGroups { get; }
        public static List<Token> Tokens { get; } = new List<Token>();


        public async Task SendMessage(string user, string message)
        {
            try
            {
                var group = getGroup(Context.ConnectionId).Name;
            }
            catch (InvalidOperationException)
            {
                await Clients.Caller.SendCoreAsync(ReceiveMethod, 
                    new object[] { "Nutzer ist in keiner Gruppe. Erst joinen!" });
            }

            if (message[0] == '/')
            {
                var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();

                var Timon = args.Any(x => x == "hallo");

                var ident = args.First().Replace("/", "");
                if (args.Count > 0) args.RemoveAt(0);

                var ret = CommandHandler.ExecuteCommand(new Command
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
                        await Clients.Caller.SendAsync(ReceiveMethod, ret.message);
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
                var group = getGroup(Context.ConnectionId).Name;
                return Clients.Group(group).SendCoreAsync(ReceiveMethod,
                    new object[] {getUser(Context.ConnectionId).Name, message});
            }
            catch (InvalidOperationException)
            {
                return Clients.Caller.SendCoreAsync(ReceiveMethod,
                    new object[] {"Nutzer ist in keiner Gruppe. Erst joinen!"});
            }
        }

        private Group getGroup(string id)
        {
            return DsaGroups.First(x => x.Users.Exists(y => y.ConnectionId.Equals(id)));
        }

        private User getUser(string id)
        {
            return DsaGroups.First(x => x.Users.Exists(y => y.ConnectionId.Equals(id))).Users
                .First(z => z.ConnectionId.Equals(id));
        }

        public async Task GetGroups() {
            var test = await Database.GetGroups();
            foreach (var group in test.Select(x => new Group(x.Item1, x.Item2)).ToList())
                if (!DsaGroups.Exists(x => x.Name.Equals(group.Name)))
                    DsaGroups.Add(group);

            await Clients.Caller.SendCoreAsync("ListGroups", new object[] {DsaGroups.Select(x => x.SendGroup())});
            //throw new NotImplementedException("add database call to get groups");
        }

        public async Task AddGroup(string group, string password)
        {
            DsaGroups.Add(new Group(group, password));
            var Dgroup = new DSALib.Models.Database.Groups.Group {Name = group, Id = DsaGroups.Count - 1};
            //Database.AddGroup(Dgroup);
            await Clients.Caller.SendCoreAsync(ReceiveMethod, new[] {$"group {group} sucessfully added"});
            //throw new NotImplementedException("add database call to add groups");
        }

        public async Task UploadChar(string xml)
        {
            var group = getGroup(Context.ConnectionId);

            await Database.AddChar(new Character(new MemoryStream(Encoding.UTF8.GetBytes(xml))), group.Name);
            //throw new NotImplementedException("add database call to add groups");
        }

        public async Task Login(string group, string user, string hash)
        {
            //string password = System.Text.Encoding.UTF8.GetString(hash);
            if (hash == DsaGroups.First(x => x.Name == group).Password)
            {
                var gGroup = DsaGroups.First(x => x.Name.Equals(group));
                if (!gGroup.Users.Exists(x => x.Name.Equals(user)))
                {
                    await Groups.RemoveFromGroupAsync(Context.ConnectionId, "login");
                    await Groups.AddToGroupAsync(Context.ConnectionId, group);
                    gGroup.Users.Add(new User {ConnectionId = Context.ConnectionId, Name = user});
                    await SendToGroup("Ein neuer Nutzer hat die Gruppe betreten");
                    await Clients.Caller.SendAsync("LoginResponse", 0);
                    await Clients.Caller.SendAsync("PlayerStatusChanged", new[] {user, "online"});

                    Tokens.Add(new Token(group));
                    await Clients.Caller.SendAsync("Token", Tokens.Last().GetHashCode());
                    purgeTokens();
                }
                else
                {
                    await Clients.Caller.SendAsync("LoginResponse", 1);
                }
            }
            else
            {
                await Clients.Caller.SendAsync("LoginResponse", 2);
                //await Clients.Caller.SendAsync(receiveMethod, "Falsches Passwort!");
            }
        }

        private void purgeTokens()
        {
            Tokens.RemoveAll(x => !x.IsValid());
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
            if (DsaGroups.Exists(x => x.Users.Exists(y => y.ConnectionId == Context.ConnectionId)))
                try
                {
                    var group = getGroup(Context.ConnectionId);


                    var user = getUser(Context.ConnectionId);

                    await Clients.Caller.SendAsync("PlayerStatusChanged", new[] {user.Name, "offline"});
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