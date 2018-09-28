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
        public async Task SendMessage(string user, string message)
        {
            string group = "default";
            var args = message.Split(' ', StringSplitOptions.RemoveEmptyEntries).ToList();
            var ident = args.First().Replace("!", ""); 
            if(args.Count>0){args.RemoveAt(0);}
            
            await SendToGroup(group, user, Commands.CommandHandler.ExecuteCommand(new Command{CharId = 0,CmdIdentifier = ident, CmdTexts = args, Name = user}));
        }

        private Task SendToGroup(string group, string user, string message)
        {
            return Clients.Group(group).SendCoreAsync("ReceiveMessage", new object[] { user, message });
        }

        public async Task GetGroups()
        {
            await  Clients.Caller.SendCoreAsync("ListGroups", new object[] { "TheCrew", "Testdata" });
        }

        public async Task Login(string group, string password)
        {
            if (password == "valid")
            {
                await Groups.AddToGroupAsync(Context.ConnectionId, group);
            }

            await SendToGroup(group, "", "Ein neuer Nutzer hat die Gruppe betreten");
        }

        public async Task Disconnect(string group, string user)
        {
            await Groups.RemoveFromGroupAsync(Context.ConnectionId, group);

            await SendToGroup(group, user, "Connection lost");
        }

    }
}
