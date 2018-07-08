using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.IO;
    using System.Net;
    using System.Net.Http;

    using DiscoBot.Auxiliary;

    using Discord.Commands;

    public class SaveCommand : ModuleBase
    {
        [Command("load"), Summary("Load Session")]
        public async Task LoadSessionAsync([Remainder, Summary("Session Name")] string name = "")
        {
            if (name.Equals("?") || name.Equals(string.Empty))
            {
                await this.ReplyAsync($"Gespeicherte Sessions:");
                await this.ReplyAsync(this.ListSessions());
                return;
            }

            var path = DSA_Game.Save.Session.DirectoryPath + @"\" + name;

            var files = Directory.GetFiles(path);
            var session = files.OrderByDescending(x => Convert.ToInt32(x.Split('-').Last().Split('.').First())).First();
            Dsa.Session = Session.Load(session);

            await this.ReplyAsync($"{name} wurde geladen");
        }

        [Command("save", RunMode = RunMode.Async), Summary("Save Session")]
        public async Task SessionSaveAsync([Remainder, Summary("Session Name")] string name = "")
        {
            var sendFile = this.Context.Channel.SendWebFile("https://cdn.discordapp.com/attachments/377123019673567232/465615882048110603/giphy.gif");

            if (name.Equals("?") || name.Equals(string.Empty))
            {
                await this.ReplyAsync($"Gespeicherte Sessions:");
                await this.ReplyAsync(this.ListSessions());
                return;
            }

            var path = DSA_Game.Save.Session.DirectoryPath + @"\" + name;
            if (Directory.Exists(path))
            {
                var files = Directory.GetFiles(path);
                int current = files.Max(x => Convert.ToInt32(x.Split('-').Last().Split('.').First()));
                Dsa.Session.Save(path + "\\" + name + $"-{++current}.json");
            }
            else
            {
                Directory.CreateDirectory(path);
                Dsa.Session.Save(path + "\\" + name + $"-0.json");
            }

            await this.ReplyAsync($"{name} wurde gespeichert");
            await sendFile;
        }

        private string[] ListSessions()
        {
            return Directory.GetDirectories(Session.DirectoryPath);
        }


    }
}
