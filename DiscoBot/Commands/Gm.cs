namespace DiscoBot.Commands
{
    using System.Linq;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;

    using Discord.Commands;
    using Discord.WebSocket;

    public class Gm : ModuleBase
    {
        public static string CheckCommand(string name, CommandTypes command, string waffe, int erschwernis = 0)
        {
            var comp = new SpellCorrect();
            var chr = Dsa.Chars.OrderBy(x => comp.Compare(x.Name, name)).First();
            switch (command)
            {
                case CommandTypes.Talent:
                    return chr.TestTalent(waffe, erschwernis);
                case CommandTypes.Eigenschaft:
                    return chr.TestEigenschaft(waffe, erschwernis);
                case CommandTypes.Angriff:
                    return chr.Angriff(waffe, erschwernis);
                case CommandTypes.Parade:
                    return chr.Parade(waffe, erschwernis);
                case CommandTypes.Fernkampf:
                    return chr.Fernkampf(waffe, erschwernis);
            }

            return $"{name} verwendet {waffe}";
        }

        [Command("gm"), Summary("Führt eine probe aus")]
        [Alias("GM", "as", "As", "als")]
        public async Task ProbeAsync([Summary("Fernkampfwaffe")] string name, string command, string waffe, int erschwernis = 0)
        {
            if (!((SocketGuildUser)this.Context.User).Roles.ToList().Exists(v => v.Name.Equals("Meister")))
            {
                await this.ReplyAsync("```xl\n Keine ausreichenden Berechtigunen\n```");
                return;
            }

            command = command.ToLower();
            string res = this.Test(name, command, waffe, erschwernis);

            if (Dsa.GeneralContext != null && Dsa.GeneralContext.Channel.Id != this.Context.Channel.Id)
            {
                    await Dsa.GeneralContext.Channel.SendMessageAsync("```xl\n" + res + "\n```");
            }

            await this.ReplyAsync("```xl\n" + res + "\n```");
        }

        private string Test(string name, string command, string waffe, int erschwernis = 0)
        {
            string res;
            switch (command)
            {
                case "f":
                case "fern":
                case "fernkampf":
                    res = CheckCommand(name, CommandTypes.Fernkampf, waffe, erschwernis);
                    break;
                case "t":
                case "ta":
                case "talent":
                case "talente":
                    res = CheckCommand(name, CommandTypes.Talent, waffe, erschwernis);
                    break;
                case "e":
                case "ei":
                case "eigenschaft":
                    res = CheckCommand(name, CommandTypes.Eigenschaft, waffe, erschwernis);
                    break;
                case "z":
                case "za":
                case "zauber":
                case "magie":
                case "m":
                    res = CheckCommand(name, CommandTypes.Talent, waffe, erschwernis);
                    break;
                case "a":
                case "at":
                case "an":
                case "angrif":
                case "angriff":
                    res = CheckCommand(name, CommandTypes.Angriff, waffe, erschwernis);
                    break;
                case "p":
                case "pa":
                case "parade":
                    res = CheckCommand(name, CommandTypes.Parade, waffe, erschwernis);
                    break;
                default:
                    res = $"Kommando {command} nicht gefunden";
                    break;
            }

            return res;
        }
    }
}
