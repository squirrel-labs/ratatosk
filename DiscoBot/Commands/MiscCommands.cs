using System;
using System.Collections.Generic;
using System.Runtime.Remoting.Contexts;
using System.Text;

using DiscoBot.Auxiliary;

using Discord;
using Discord.Commands;
using Discord.WebSocket;

namespace DiscoBot.Commands
{
    using System;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.IO;
    using System.Linq;
    using System.Net;
    using System.Net.Http;
    using System.Net.Mime;
    using System.Text;
    using System.Threading;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;
    using DiscoBot.DSA_Game;

    using Discord;
    using Discord.Commands;

    public class MiscCommands : ModuleBase
    {
        [Command("r"), Summary("Würfelt ")]
        [Alias("R", "Roll", "roll", "Würfle")]
        public Task RollAsync([Remainder, Summary("Weapon")] string roll)
        {
            //return this.ReplyAsync("```xl\n" + new Auxiliary.Calculator.StringSolver(roll).Solve() + "\n```");
            return this.ReplyAsync("```xl\n" + RandomMisc.Roll(roll) + "\n```");
        }

        [Command("rd"), Summary("Würfel Dennis ")]
        public Task RollDennisAsync([Remainder, Summary("Weapon")] string roll)
        {
            return this.ReplyAsync("```xl\n" + new Auxiliary.Calculator.StringSolver(roll).Solve() + "\n```");
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

        [Command("liebe"), Summary("Echos a message.")]
        [Alias("Liebe", "<3", "love")]
        public async Task LoveAsync()
        {
            Random rand = new Random();
            var user = Context.Channel.GetUsersAsync().ToList().Result.ToList().First().Where(x=>x.Status!= UserStatus.Offline).OrderBy(x => rand.Next()).First();
            await this.ReplyAsync(":heart: :heart: :heart: Verteilt die Liebe! :heart: :heart: :heart: \n Besondere Liebe geht an " + user.Username);
            //await this.ReplyAsync("!liebe");
        }

        [Command("maul"), Summary("Echos a message.")]
        public Task MaulAsync()
        {
            return this.ReplyAsync("Maul...? Du meintest doch sicher Maulwürfe oder? \n:heart: :heart: :heart: \nGanz viel Liebe für Maulwürfe !\n:heart: :heart: :heart:");

        }

        [Command("report"), Summary("Report a Tweet")]
        public async Task ReportAsync([Remainder, Summary("Link")] string link)
        {
            var content = new System.Net.Http.StringContent(link);
            
            using (HttpClient client = new HttpClient())
            {
                var response = await client.PostAsync("http://www.example.com/recepticle.aspx", content);
            }

            await this.ReplyAsync($"Dein report wurde hinzugefügt");
        }

        [Command("match"), Summary("Tinder.")]
        [Alias("mach","pass", "passt")]
        public Task TinderAsync(string s1, string s2)
        {

            var sc = new SpellCorrect();
            var rand = new System.Random((s1+s2).GetHashCode());

            var wert = Math.Log10(Math.Floor(1000.0 * (SpellCorrect.CompareExact(s1, s2) + rand.NextDouble() * 10.0)) / 1000.0);
            wert = ((wert * 100.0) < 100.0 ? wert * 100.0 : 100.0 - wert);
            wert = wert < 0 ? -wert : wert;
            return this.ReplyAsync($"Ihr passt zu {Math.Floor(100.0 * wert )/ 100.0}% zusammen");

        }

        [Command("reddit"), Summary("Reddit.")]
        public Task RedditAsync()
        {
            return this.ReplyAsync($"Ein Archiv der Vergangenen Aktionen findet man hier: https://www.reddit.com/r/ReconquistaInternet/");

        }

        [Command("compare"), Summary("Echos a message.")]
        public async Task KickAsync()
        {
            //await this.Context.Guild.DownloadUsersAsync();
            var users =  Context.Guild.GetUsersAsync(CacheMode.AllowDownload);
            var test = File.ReadAllLines("RG.txt");
            await users;
            var us = users.Result.Select(x => x.Username);
            
            var lines = test.Where(x => !x.Equals(string.Empty)).ToList();
            

            var sc = new SpellCorrect();

            var res = new List<string>();

            foreach (string line in lines)
            {
                var best = us.OrderBy(user => sc.Compare(user, line)).First();

                double fit = sc.Compare(best, line);

                if (fit < SpellCorrect.ErrorThreshold - 20000)
                {
                    if (fit.Equals(0))
                    {
                        res.Add($"@\t{best} !!! => {line}");
                    }
                    else
                    {
                        res.Add($"-\t{best} hat Ähnlichkeit mit: {line}");
                    }
                }
            }

            var sb = new StringBuilder();
            foreach (string re in res)
            {
                if (sb.Length + re.Length > 1798)
                {
                    await this.ReplyTimedAsync(sb.ToString(), TimeSpan.FromSeconds(90));
                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            if(Permissions.Check(this.Context, new []{"Admin", "Mod"}))
            await this.ReplyTimedAsync(sb.ToString(), TimeSpan.FromSeconds(90));

            //await this.ReplyAsync($"{count} Duplikate gefunden");

        }


        [Command("clear"), Summary("Cleans up messages.")]
        public async Task DeleteAsync(int count)
        {
            var messagesAsync = Context.Channel.GetMessagesAsync(count);
            Task.WaitAll(messagesAsync.ToArray());
            var list = messagesAsync.ToEnumerable().ToList();
            var messages = new List<IMessage>();
            foreach (var task in list)
            {
                messages.AddRange(task.ToList());
            }

            if (Permissions.Check(Context, new[] { "Admin", "Mod", "Meister" }))
            {
                
                var waiters = new List<Task>();
                foreach (var message in messages)
                {
                    waiters.Add((message as IUserMessage).DeleteAsync());
                }

                Task.WaitAll(waiters.ToArray());
            }
            
        }

        [Command("check"), Summary("Echos a message.")]
        [Alias("Check")]
        public async Task CheckAsync(string quarry)
        {
            var perm = new List<string> { "Admin", "Mod", "Privatpolizei" };

            Permissions.Test(this.Context, perm.ToArray());

            var test = File.ReadAllLines("RG.txt");

            var lines = test.Where(x => !x.Equals(string.Empty)).ToList();


            var sc = new SpellCorrect();
            var count = lines.OrderBy(line => sc.Compare(quarry, line)).First();

            var fit = sc.Compare(count, quarry);

            string Antwort;

            if (fit < SpellCorrect.ErrorThreshold - 20000)
            {
                Antwort= $"```xl\nAuf anderem Server Match gefunden: {count}";
            }
            else
            {
                Antwort = $"```xl\nAuf anderem Server Kein Match gefunden: {quarry}";
            }


            var users = Context.Guild.GetUsersAsync(CacheMode.AllowDownload);
            await users;
            var us = users.Result.Select(x => x.Username);
            
            sc = new SpellCorrect();
            count = us.OrderBy(line => sc.Compare(quarry, line)).First();

            fit = sc.Compare(count, quarry);

            if (fit < SpellCorrect.ErrorThreshold - 20000)
            {
                Antwort = Antwort + $"\nAuf unserem Server Match gefunden: {count}\n```";
            }
            else
            {
                Antwort = Antwort + $"\nAuf unserem Server Kein Match gefunden: {quarry} \n```";
            }

            await ReplyAsync(Antwort);

        }
    }
}
