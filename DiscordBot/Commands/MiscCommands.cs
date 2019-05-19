using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using DiscoBot.Auxiliary;
using Discord;
using Discord.Commands;
using DiscordBot.Auxiliary;

namespace DiscordBot.Commands
{
    public class MiscCommands : ModuleBase
    {
        [Command("r")]
        [Summary("Würfelt ")]
        [Alias("R", "Roll", "roll", "Würfle")]
        public Task RollAsync([Remainder] [Summary("Weapon")] string roll)
        {
            //return this.ReplyAsync("```xl\n" + new Auxiliary.Calculator.StringSolver(roll).Solve() + "\n```");
            return ReplyAsync("```xl\n" + RandomMisc.Roll(roll) + "\n```");
        }


        [Command("say")]
        [Summary("Echos a message.")]
        [Alias("s")]
        public Task SayAsync([Remainder] [Summary("The text to echo")]
            string echo)
        {
            return ReplyAsync(echo);
        }

        [Command("liebe")]
        [Summary("Echos a message.")]
        [Alias("Liebe", "<3", "love")]
        public async Task LoveAsync()
        {
            var rand = new Random();
            var user = Context.Channel.GetUsersAsync().ToList().Result.ToList().First()
                .Where(x => x.Status != UserStatus.Offline).OrderBy(x => rand.Next()).First();
            await ReplyAsync(
                ":heart: :heart: :heart: Verteilt die Liebe! :heart: :heart: :heart: \n Besondere Liebe geht an " +
                user.Username);
            //await this.ReplyAsync("!liebe");
        }

        [Command("maul")]
        [Summary("Echos a message.")]
        public Task MaulAsync()
        {
            return ReplyAsync(
                "Maul...? Du meintest doch sicher Maulwürfe oder? \n:heart: :heart: :heart: \nGanz viel Liebe für Maulwürfe !\n:heart: :heart: :heart:");
        }


        [Command("match")]
        [Summary("Tinder.")]
        [Alias("mach", "pass", "passt")]
        public Task TinderAsync(string s1, string s2)
        {
            var rand = new Random((s1 + s2).GetHashCode());

            var wert = Math.Log10(Math.Floor(1000.0 * (SpellCorrect.CompareExact(s1, s2) + rand.NextDouble() * 10.0)) /
                                  1000.0);
            wert = wert * 100.0 < 100.0 ? wert * 100.0 : 100.0 - wert;
            wert = wert < 0 ? -wert : wert;
            return ReplyAsync($"Ihr passt zu {Math.Floor(100.0 * wert) / 100.0}% zusammen");
        }

        [Command("reddit")]
        [Summary("Reddit.")]
        public Task RedditAsync()
        {
            return ReplyAsync(
                "Ein Archiv der Vergangenen Aktionen findet man hier: https://www.reddit.com/r/ReconquistaInternet/");
        }

        [Command("compare")]
        [Summary("Echos a message.")]
        public async Task KickAsync()
        {
            //await this.Context.Guild.DownloadUsersAsync();
            var users = Context.Guild.GetUsersAsync();
            var test = File.ReadAllLines("RG.txt");
            await users;
            var us = users.Result.Select(x => x.Username);

            var lines = test.Where(x => !x.Equals(string.Empty)).ToList();


            var sc = new SpellCorrect();

            var res = new List<string>();

            foreach (var line in lines)
            {
                var best = us.OrderBy(user => sc.Compare(user, line)).First();

                double fit = sc.Compare(best, line);

                if (!(fit < SpellCorrect.ErrorThreshold - 20000)) continue;
                res.Add(fit.Equals(0) ? $"@\t{best} !!! => {line}" : $"-\t{best} hat Ähnlichkeit mit: {line}");
            }

            var sb = new StringBuilder();
            foreach (var re in res)
            {
                if (sb.Length + re.Length > 1798)
                {
                    await CommandHelper.ReplyTimedAsync(this, sb.ToString(), TimeSpan.FromSeconds(90));
                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            if (Permissions.Check(Context, new[] {"Admin", "Mod"}))
                await CommandHelper.ReplyTimedAsync(this, sb.ToString(), TimeSpan.FromSeconds(90));

            //await this.ReplyAsync($"{count} Duplikate gefunden");
        }


        [Command("clear")]
        [Summary("Cleans up messages.")]
        public void DeleteAsync(int count)
        {
            var messagesAsync = Context.Channel.GetMessagesAsync(count);
            if (messagesAsync != null)
            {
                Task.WaitAll(messagesAsync.ToArray());
                var list = messagesAsync.ToEnumerable().ToList();
                var messages = new List<IMessage>();
                foreach (var task in list) messages.AddRange(task.ToList());

                if (Permissions.Check(Context, new[] {"Admin", "Mod", "Meister"}))
                {
                    var waiters = new List<Task>();
                    foreach (var message in messages) waiters.Add(((IUserMessage) message).DeleteAsync());

                    Task.WaitAll(waiters.ToArray());
                }
            }
        }

        [Command("check")]
        [Summary("Echos a message.")]
        [Alias("Check")]
        public async Task CheckAsync(string quarry)
        {
            var perm = new List<string> {"Admin", "Mod", "Privatpolizei"};

            Permissions.Test(Context, perm.ToArray());

            var test = File.ReadAllLines("RG.txt");

            var lines = test.Where(x => !x.Equals(string.Empty)).ToList();


            var sc = new SpellCorrect();
            var count = lines.OrderBy(line => sc.Compare(quarry, line)).First();

            var fit = sc.Compare(count, quarry);

            string antwort;

            antwort = fit < SpellCorrect.ErrorThreshold - 20000
                ? $"```xl\nAuf anderem Server Match gefunden: {count}"
                : $"```xl\nAuf anderem Server Kein Match gefunden: {quarry}";


            var users = Context.Guild.GetUsersAsync();
            await users;
            var us = users.Result.Select(x => x.Username);

            sc = new SpellCorrect();
            count = us.OrderBy(line => sc.Compare(quarry, line)).First();

            fit = sc.Compare(count, quarry);

            antwort = fit < SpellCorrect.ErrorThreshold - 20000
                ? $"{antwort}\nAuf unserem Server Match gefunden: {count}\n```"
                : $"{antwort}\nAuf unserem Server Kein Match gefunden: {quarry} \n```";

            await ReplyAsync(antwort);
        }
    }
}