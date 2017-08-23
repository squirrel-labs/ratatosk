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
            relation.Add("DSA Bot", "Numeri Illuminus");
            chars.Add(new Char(@"helden\Felis.xml"));       //Savefile
            chars.Add(new Char(@"helden\Numeri.xml"));

        }
    }
    public class Info : ModuleBase
    {
        [Command("say"), Summary("Echos a message.")]
        public async Task Say([Remainder, Summary("The text to echo")] string echo)
        {
            var a = Context.User.Username;   
            
            await ReplyAsync(echo);
            
        }
    }

    public class Roll : ModuleBase
    {
        [Command("r"), Summary("Würfelt ")]
        [Alias("R", "Roll", "roll", "Würfle")]
        public async Task Say([Remainder, Summary("Weapon")] string roll)
        {

            await ReplyAsync("```xl\n**" + Misc.Roll(roll) + "**\n```");

        }
    }

    public class TestTalent : ModuleBase
    {
        [Command("t"), Summary("Würfelt ein Talent-/Zauberprobe")]
        [Alias("T", "Talent", "talent","zauber","z", "versuche")]
        public async Task Say([Remainder, Summary("Talent oder Zaubername")] string talent)
        {

            await ReplyAsync("```xl\n" + DSA.chars.Find(x=>x.name.Equals(DSA.relation[Context.User.Username])).TestTalent(talent) + "\n```");

        }
    }
    public class Angriff : ModuleBase
    {
        [Command("a"), Summary("Würfelt ein Angriff")]
        [Alias("A", "Angriff", "angriff", "attackiere_mit","attacke","Attacke")]
        public async Task Say([Remainder, Summary("Weapon")] string weapon)
        {

            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Angriff(weapon) + "\n```");

        }
    }
    public class Parade : ModuleBase
    {
        // ~say hello -> hello
        [Command("p"), Summary("Würfelt eine Parade Probe")]
        [Alias("P", "Parade", "parade","pariere_mit")]
        public async Task Say([Remainder, Summary("Parade Weapon")] string talent)
        {
            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Parade(talent) + "\n```");

        }
    }
    public class Fernkampf : ModuleBase
    {
        // ~say hello -> hello
        [Command("f"), Summary("Führt eine Fernkampfprobe aus")]
        [Alias("F", "fernkampf", "Fernkampf", "schieße", "schieße_mit")]
        public async Task Say([Summary("Fernkampfwaffe")] string talent,int erschwernis=0)
        {
            // ReplyAsync is a method on ModuleBase

            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.name.Equals(DSA.relation[Context.User.Username])).Fernkampf(talent,erschwernis) + "\n```");

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
