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
    using System.ComponentModel;
    using System.Diagnostics.CodeAnalysis;
    using System.Threading;

    public class Info : ModuleBase
    {
        [Command("say"), Summary("Echos a message.")]
        [Alias("s")]
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
            await ReplyAsync("```xl\n" + Misc.Roll(roll) + "\n```");
        }
    }

    public class SetGeneral : ModuleBase
    {
        [Command("general"), Summary("Set General ")]
        public async Task Say([Remainder, Summary("Set General")] int i = 0)
        {
            DSA.GeneralContext = this.Context;
            await this.Context.Channel.SendMessageAsync($"```xl\n Der Dachs hat in '{this.Context.Channel.Name}' ein Zuhause gefunden. Gm Nachrichten werden nun auch in diesem Channel gepostet. \n```");
        }
    }

    [SuppressMessage("ReSharper", "PublicMembersMustHaveComments", Justification = "OK")]
    public class TestTalent : ModuleBase
    {
        [Command("t"), Summary("Würfelt ein Talent-/Zauberprobe")]
        [Alias("T", "Talent", "talent", "zauber", "z", "versuche")]
        public async Task Say([Summary("Talent oder Zaubername")] string talent, int erschwernis = 0)
        {
            string res = Gm.CheckCommand(DSA.relation[Context.User.Username], Commands.Talent, talent, erschwernis);
            await this.ReplyAsync("```xl\n" + res + "\n```");
            
            var tmessages = this.Context.Channel.GetMessagesAsync(10);
            Task.WaitAll(tmessages.ToArray());
            var list = tmessages.ToEnumerable().ToList();
            var messages = new List<IMessage>();
            foreach (var task in list)
            {
                messages.AddRange(task.ToList());
            }

            //await this.Context.Channel.DeleteMessagesAsync(messages.Where(x => x.Content[0].Equals('!') && (x.Author as SocketGuildUser).Roles.ToList().Exists(v => v.Name.Equals("Meister"))));
        }
    }

    public class TestEigenschaft : ModuleBase
    {
        [Command("e"), Summary("Würfelt eine Eifenschaftsprobe")]
        [Alias("E", "Eigenschaft", "eigenschaft", "eigen")]
        public async Task Say([Summary("Eigenschafts kürzel und Erschwernis")] string talent, int erschwernis = 0)
        {
            var chr = DSA.chars.Find(x => x.Name.Equals(DSA.relation[Context.User.Username]));
            string res = chr.TestEigenschaft(talent, erschwernis);
            await this.ReplyAsync("```xl\n" + res + "\n```");

        }
    }

    public class Angriff : ModuleBase
    {
        [Command("a"), Summary("Würfelt ein Angriff")]
        [Alias("At", "at", "Angriff", "angriff", "attackiere_mit", "attacke", "Attacke")]
        public async Task Say([Summary("Weapon")] string weapon, int erschwernis = 0)
        {
            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.Name.Equals(DSA.relation[Context.User.Username])).Angriff(weapon, erschwernis) + "\n```");

        }
    }
    public class Parade : ModuleBase
    {
        // ~say hello -> hello
        [Command("p"), Summary("Würfelt eine Parade Probe")]
        [Alias("P", "Parade", "parade", "pariere_mit")]
        public async Task Say([Summary("Parade Weapon")] string talent, int erschwernis = 0)
        {
            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.Name.Equals(DSA.relation[Context.User.Username])).Parade(talent, erschwernis) + "\n```");

        }
    }
    public class Fernkampf : ModuleBase
    {
        // ~say hello -> hello
        [Command("f"), Summary("Führt eine Fernkampfprobe aus")]
        [Alias("F", "fernkampf", "Fernkampf", "schieße", "schieße_mit")]
        public async Task Say([Summary("Fernkampfwaffe")] string waffe, int erschwernis = 0)
        {
            // ReplyAsync is a method on ModuleBase
            await ReplyAsync("```xl\n" + DSA.chars.Find(x => x.Name.Equals(DSA.relation[Context.User.Username])).Fernkampf(waffe, erschwernis) + "\n```");
        }
        public async Task Say([Summary("Fernkampfwaffe")] string charName, string waffe, ICommandContext context, int erschwernis = 0)
        {
            // ReplyAsync is a method on ModuleBase
            await context.Channel.SendMessageAsync("Hello World\n");
            await context.Channel.SendMessageAsync("```xl\n" + DSA.chars.Find(x => x.Name.Equals(charName)).Fernkampf(waffe, erschwernis) + "\n```");
        }
    }

    public class Voice : ModuleBase
    {
        [Command("join")]
        public async Task JoinChannel(IVoiceChannel channel = null)
        {
            // Get the audio channel
            channel = channel ?? (this.Context.User as IGuildUser)?.VoiceChannel;
            if (channel == null)
            {
                await this.Context.Channel.SendMessageAsync(
                    "User must be in a voice channel, or a voice channel must be passed as an argument.");
                return;
            }

            // For the next step with transmitting audio, you would want to pass this Audio Client in to a service.
            var audioClient = await channel.ConnectAsync();
            
        }
    }

    [Group("gmtr")]
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

    public class List : ModuleBase
    {
        // ~say hello -> hello
        [Command("list"), Summary("gibt eine Auflistung  aus")]
        public async Task Say([Summary("Aktion")] string prop)
        {
            var res = new List<string>();
            switch (prop.ToLower())
            {
                case "chars":
                case "Chars":
                    res.AddRange(DSA.chars.Select(x => x.Name));
                    break;
                case "t":
                case "ta":
                case "talent":
                case "zauber":
                    res.AddRange(
                        ((Character)DSA.chars.Find(x => x.Name.Equals(DSA.relation[this.Context.User.Username])))
                        .Talente.Select(s => s.name + "\t " + s.value + "\t " + s.probe));
                    break;
                case "w":
                case "waffe":
                case "waffen":
                    res.AddRange(
                        ((Character)DSA.chars.Find(x => x.Name.Equals(DSA.relation[this.Context.User.Username])))
                        .Kampftalente.Select(s => s.name));
                    break;
                case "fern":
                    res.AddRange(
                        ((Character)DSA.chars.Find(x => x.Name.Equals(DSA.relation[this.Context.User.Username])))
                        .Talente.Select(s => s.name));
                    break;
                case "v":
                case "vt":
                case "vor":
                case "vorteil":
                    res.AddRange(
                        ((Character)DSA.chars.Find(x => x.Name.Equals(DSA.relation[this.Context.User.Username])))
                        .Vorteile
                        .Select(s => s.name + "\t " + (s.value == 0 ? "" : s.value.ToString())));
                    break;

                default:
                    res.Add($"Kommando {prop} nicht gefunden");
                    break;
            }

            //await this.ReplyAsync(res.Aggregate((seed, next) => seed + "\n" + next));
            var sb = new StringBuilder();
            foreach (string re in res)
            {
                if (sb.Length + re.Length > 1798)
                {
                    await this.ReplyTimed(sb.ToString(), TimeSpan.FromSeconds(90));
                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            await this.ReplyTimed(sb.ToString(), TimeSpan.FromSeconds(90));
        }

    }

    public class Gm : ModuleBase
    {
        // ~say hello -> hello
        [Command("gm"), Summary("Führt eine probe aus")]
        [Alias("GM", "as", "As", "als")]
        public async Task Say([Summary("Fernkampfwaffe")] string name, string command, string waffe, int erschwernis = 0)
        {
            if (!(this.Context.User as SocketGuildUser).Roles.ToList().Exists(v => v.Name.Equals("Meister")))
            {
                await ReplyAsync("```xl\n Keine ausreichenden Berechtigunen\n```");
                return;
            }

            command = command.ToLower();
            string res;
            switch (command)
            {
                case "f":
                case "fern":
                case "fernkampf":
                    res = CheckCommand(name, Commands.Fernkampf, waffe, erschwernis);
                    break;
                case "t":
                case "ta":
                case "talent":
                    res = CheckCommand(name, Commands.Talent, waffe, erschwernis);
                    break;
                case "e":
                case "ei":
                case "eigenschaft":
                    res = CheckCommand(name, Commands.Eigenschaft, waffe, erschwernis);
                    break;
                case "z":
                case "za":
                case "zauber":
                case "magie":
                case "m":
                    res = CheckCommand(name, Commands.Talent, waffe, erschwernis);
                    break;
                case "a":
                case "at":
                case "an":
                case "angrif":
                case "angriff":
                    res = CheckCommand(name, Commands.Angriff, waffe, erschwernis);
                    break;
                case "p":
                case "pa":
                case "parade":
                    res = CheckCommand(name, Commands.Parade, waffe, erschwernis);
                    break;
                case "talente":
                    throw new NotImplementedException();
                default:
                    res = $"Kommando {command} nicht gefunden";
                    break;
            }

            if (DSA.GeneralContext != null)
            {
                if (DSA.GeneralContext.Channel.Id != Context.Channel.Id)
                {
                    await DSA.GeneralContext.Channel.SendMessageAsync("```xl\n" + res + "\n```");
                }

                await ReplyAsync("```xl\n" + res + "\n```");
            }
            else
            {
                await ReplyAsync("```xl\n" + res + "\n```");
            }
        }

        public static string CheckCommand(string name, Commands command, string waffe, int erschwernis = 0)
        {
            var comp = new SpellCorrect();
            var chr = DSA.chars.OrderBy(x => comp.Compare(x.Name, name)).First();
            switch (command)
            {
                case Commands.Talent:
                    return chr.TestTalent(waffe, erschwernis);
                case Commands.Eigenschaft:
                    return chr.TestEigenschaft(waffe, erschwernis);
                case Commands.Angriff:
                    return chr.Angriff(waffe, erschwernis);
                case Commands.Parade:
                    return chr.Parade(waffe, erschwernis);
                case Commands.Fernkampf:
                    return chr.Fernkampf(waffe, erschwernis);
            }

            return $"{name} verwendet {waffe}";
        }
    }

    public class GenerateNpc : ModuleBase
    {
        // ~say hello -> hello
        [Command("npc"), Summary("Erstellt ein NPC")]
        [Alias("Npc", "NPc", "NPC", "nPC")]
        public async Task Say([Summary("Create Random")] string npcName, int mean = 9, int stDv = 1)
        {
            DSA.chars.Add(new NPC(npcName, mean, stDv));
            await this.ReplyAsync($"{npcName} wurde zufällig generiert");
        }
    }

    public class CopyNpc : ModuleBase
    {
        // ~say hello -> hello
        [Command("npc"), Summary("Erstellt ein NPC")]
        [Alias("Npc", "NPc", "NPC", "nPC")]
        public async Task Say([Summary("Create Copy")] string npcName, string source, int stDv = 1)
        {
            if (DSA.chars.Exists(x => x.Name.Equals(npcName)))
            {
                throw new Exception("Char gibt es schon");
            }

            var comp = new SpellCorrect();
            var chr = DSA.chars.OrderBy(x => comp.Compare(x.Name, source)).First();
            DSA.chars.Add(new Character(chr as Character, npcName, stDv));
            await ReplyAsync($"{npcName} wurde als variierte Kopie von {source} erstellt");
        }
    }

    public class NpcAction : ModuleBase
    {
        // ~say hello -> hello
        [Command("npc"), Summary("Führt eine NPC-Probe aus")]
        [Alias("Npc", "NPc", "NPC", "nPC")]
        public async Task Say([Summary("Aktion")] string NpcName, string command, string Aktion, int erschwernis = 0)
        {
            string test = "";
        }
    }


    public enum Commands
    {
        Talent,
        Eigenschaft,
        Angriff,
        Parade,
        Fernkampf,
        KeinChar
    }
}
