namespace DiscoBot.Commands
{
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;

    using Discord.Commands;

    public class MiscCommands : ModuleBase
    {
        [Command("r"), Summary("Würfelt ")]
        [Alias("R", "Roll", "roll", "Würfle")]
        public Task RollAsync([Remainder, Summary("Weapon")] string roll)
        {
            return this.ReplyAsync("```xl\n" + RandomMisc.Roll(roll) + "\n```");
        }

        [Command("general"), Summary("Set General ")]
        public Task SetGeneralAsync([Remainder, Summary("Set General")] int i = 0)
        {
            Dsa.GeneralContext = this.Context;
            return this.Context.Channel.SendMessageAsync($"```xl\n Der Dachs hat in '{this.Context.Channel.Name}' ein Zuhause gefunden. Gm Nachrichten werden nun auch in diesem Channel gepostet. \n```");
        }

        [Command("say"), Summary("Echos a message.")]
        [Alias("s")]
        public Task SayAsync([Remainder, Summary("The text to echo")] string echo)
        {
            return this.ReplyAsync(echo);
        }

        [Command("spot"), Summary("Echos a message.")]
        [Alias("spotify")]
        public Task SpotiAsync([Remainder, Summary("The text to echo")] string echo)
        {
            var test = new Spotify.WebClient();
            return this.ReplyAsync(string.Join("\n", test.GetPlaylist("")));
        }
    }
}
