using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;


namespace DiscoBot
{
    class Commands
    {
    }
    public class Info : ModuleBase
    {
        // ~say hello -> hello
        [Command("say"), Summary("Echos a message.")]
        public async Task Say([Remainder, Summary("The text to echo")] string echo)
        {
            

            // ReplyAsync is a method on ModuleBase
            await ReplyAsync(echo);
            
        }
    }

    public class Abfrage : ModuleBase
    {
        // ~say hello -> hello
        [Command("t"), Summary("tests a talent.")]
        public async Task Say([Remainder, Summary("The text to echo")] string talent)
        {
            //
            //a.talente.First(x=>)
            

            // ReplyAsync is a method on ModuleBase

            await ReplyAsync(talent);

        }
    }

    [Group("sample")]
    public class Sample : ModuleBase
    {
        // ~sample square 20 -> 400
        [Command("square"), Summary("Squares a number.")]
        public async Task Square([Summary("The number to square.")] int num)
        {
            // We can also access the channel from the Command Context.
            await Context.Channel.SendMessageAsync($"{num}^2 = {Math.Pow(num, 2)}");
        }
        
        [Command("userinfo"), Summary("Returns info about the current user, or the user parameter, if one passed.")]
        [Alias("user", "whois")]
        public async Task UserInfo([Summary("The (optional) user to get info for")] IUser user = null)
        {
            var userInfo = user ?? Context.Client.CurrentUser;
            await ReplyAsync($"{userInfo.Username}#{userInfo.Discriminator}");
        }
    }

}
