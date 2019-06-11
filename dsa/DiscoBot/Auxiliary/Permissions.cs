using System.Collections.Generic;
using System.Linq;
using Discord.Commands;
using Discord.WebSocket;

namespace DiscoBot.Auxiliary
{
    public static class Permissions
    {
        public static bool Check(ICommandContext c, string role)
        {
            return ((SocketGuildUser) c.User).Roles.ToList().Exists(v => v.Name.Equals(role));
        }

        public static bool Check(ICommandContext c, IEnumerable<string> roles)
        {
            return roles.Any(role => ((SocketGuildUser) c.User).Roles.ToList().Exists(v => v.Name.Equals(role)));
        }

        public static bool Test(ICommandContext c, string role)
        {
            if (Check(c, role)) return true;
            c.Channel.SendMessageAsync("```xl\n Keine ausreichenden Berechtigungen\n```").Wait();
            return false;
        }

        public static void Test(ICommandContext c, string[] roles)
        {
            if (!Check(c, roles)) c.Channel.SendMessageAsync("```xl\n Keine ausreichenden Berechtigungen\n```").Wait();
        }
    }
}