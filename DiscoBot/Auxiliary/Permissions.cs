using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.Auxiliary
{
    using Discord.Commands;
    using Discord.WebSocket;

    public static class Permissions
    {
        public static bool Check(ICommandContext c, string role)
        {
            return ((SocketGuildUser)c.User).Roles.ToList().Exists(v => v.Name.Equals(role));
        }

        public static bool Check(ICommandContext c, string[] roles)
        {
            return roles.Any(role => ((SocketGuildUser)c.User).Roles.ToList().Exists(v => v.Name.Equals(role)));
        }

        public static void Test(ICommandContext c, string role)
        {
            if (!Check(c, role))
            {
                c.Channel.SendMessageAsync("```xl\n Keine ausreichenden Berechtigungen\n```").Wait();
            }
        }

        public static void Test(ICommandContext c, string[] roles)
        {
            if (!Check(c, roles))
            {
                c.Channel.SendMessageAsync("```xl\n Keine ausreichenden Berechtigungen\n```").Wait();
            }
        }
    }
}
