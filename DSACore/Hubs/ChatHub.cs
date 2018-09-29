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
        private static Dictionary<string, User> UserGroup;

        public async Task SendMessage(string user, string message)
        {
            var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();
            var ident = args.First().Replace("!", ""); 
            if(args.Count>0){args.RemoveAt(0);}
            
            await SendToGroup(UserGroup[Context.ConnectionId].Group, user, Commands.CommandHandler.ExecuteCommand(new Command{CharId = 0,CmdIdentifier = ident, CmdTexts = args, Name = user}));
        }

        private Task SendToGroup(string group, string user, string message)
        {
            return Clients.Group(group).SendCoreAsync("ReceiveMessage", new object[] { user, message });
        }

        public async Task GetGroups()
        {
            await  Clients.Caller.SendCoreAsync("ListGroups", new object[] { "TheCrew", "Testdata" });
            throw new NotImplementedException("add database call to get groups");
        }

        public async Task Login(string group, string password)
        {
            if (password == "valid")
            {
                UserGroup.Add(Context.ConnectionId, new User{Group = group});
                await Groups.AddToGroupAsync(Context.ConnectionId, group);
            }

            await SendToGroup(group, "", "Ein neuer Nutzer hat die Gruppe betreten");
        }

        public async Task Disconnect()
        {
            var user = UserGroup[Context.ConnectionId];
            await Groups.RemoveFromGroupAsync(Context.ConnectionId, user.Group);

            await SendToGroup(user.Group, user.Name, "Connection lost");
        }

    }
}
