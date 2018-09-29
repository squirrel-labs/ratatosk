using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using DSACore.Models;
using Microsoft.AspNetCore.SignalR;

namespace DSACore.Hubs
{
    public class ChatHub : Hub
    {
        //private static Dictionary<string, User> UserGroup = new Dictionary<string, User>();

        private static List<Group> DSAGroups = new List<Group>(); 

        public async Task SendMessage(string user, string message)
        {
            var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();
            var ident = args.First().Replace("!", ""); 
            if(args.Count>0){args.RemoveAt(0);}
            
            await SendToGroup(getGroup(Context.ConnectionId).Name, user, Commands.CommandHandler.ExecuteCommand(new Command{CharId = 0,CmdIdentifier = ident, CmdTexts = args, Name = user}));
        }

        private Task SendToGroup(string group, string user, string message)
        {
            return Clients.Group(group).SendCoreAsync("ReceiveMessage", new object[] { user, message });
        }

        private Group getGroup(string id)
        {
            return DSAGroups.First(x => x.Users.Exists(y => y.ConnectionId.Equals(id)));
        }

        public async Task GetGroups()
        {
            await  Clients.Caller.SendCoreAsync("ListGroups", new object[] { Groups });
            throw new NotImplementedException("add database call to get groups");
        }

        public async Task AddGroup(string group, string password)
        {
            await Clients.Caller.SendCoreAsync("ListGroups", new object[] { Groups });
            throw new NotImplementedException("add database call to add groups");
        }

        public async Task Login(string group, string user, string password)
        {
            if (password == "valid")
            {
                DSAGroups.First(x=>x.Name.Equals(group)).Users.Add(new User{ConnectionId = Context.ConnectionId, Name = user});
                await Groups.AddToGroupAsync(Context.ConnectionId, group);
            }

            await SendToGroup(group, user, "Ein neuer Nutzer hat die Gruppe betreten");
        }

        public async Task Disconnect()
        {
            var group = getGroup(Context.ConnectionId);
            var user = group.Users.First(x => x.ConnectionId.Equals(Context.ConnectionId));
            group.Users.Remove(user);
            await Groups.RemoveFromGroupAsync(Context.ConnectionId, group.Name);

            await SendToGroup(group.Name, user.Name, "Connection lost");
        }

    }
}
