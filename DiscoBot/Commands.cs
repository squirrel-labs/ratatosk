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
    public static class DSA
    {
        public static Dictionary<string, string> relation = new Dictionary<string, string>(); //dictionary to match the char
        public static List<Char> chars = new List<Char>();                                      //list of all charackters
        public static void Startup()    
        {
            relation.Add("The Doctor", "Felis Exodus Schattenwald");//Relation
            relation.Add("Tardis", "Numeri Illuminus");
            chars.Add(new Char(@"helden\Felis.xml"));       //Savefile
            chars.Add(new Char(@"helden\Numeri.xml"));

        }
    }
    public class Info : ModuleBase
    {
        // ~say hello -> hello
        [Command("say"), Summary("Echos a message.")]
        public async Task Say([Remainder, Summary("The text to echo")] string echo)
        {
            var a = Context.User.Username;   

            // ReplyAsync is a method on ModuleBase
            await ReplyAsync(echo);
            
        }
    }

    public class TestTalent : ModuleBase
    {
        // ~say hello -> hello
        [Command("t"), Summary("tests a talent.")]
        public async Task Say([Remainder, Summary("The text to echo")] string talent)
        {
            // ReplyAsync is a method on ModuleBase

            await ReplyAsync("```xl\n" + DSA.chars.Find(x=>x.name.Equals(DSA.relation[Context.User.Username])).TestTalent(talent) + "\n```");

        }
    }
    public class Angriff : ModuleBase
    {
        // ~say hello -> hello
        [Command("a"), Summary("tests a attack.")]
        public async Task Say([Remainder, Summary("The text to echo")] string talent)
        {
            // ReplyAsync is a method on ModuleBase

            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Angriff(talent) + "\n```");

        }
    }
    public class Parade : ModuleBase
    {
        // ~say hello -> hello
        [Command("p"), Summary("tests a parade.")]
        public async Task Say([Remainder, Summary("The text to echo")] string talent)
        {
            // ReplyAsync is a method on ModuleBase

            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Parade(talent) + "\n```");

        }
    }
    public class Fernkampf : ModuleBase
    {
        // ~say hello -> hello
        [Command("f"), Summary("tests a shot.")]
        public async Task Say([Remainder, Summary("The text to echo")] string talent)
        {
            // ReplyAsync is a method on ModuleBase

            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Fernkampf(talent) + "\n```");

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
