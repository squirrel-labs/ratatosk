using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.IO;
    using System.Net.Http;

    using DiscoBot.Auxiliary;

    using Discord.Commands;

    public class SaveCommand : ModuleBase
    {
        [Command("save"), Summary("Save Session")]
        public async Task ReportAsync()
        {
            

            await this.ReplyAsync($"Dein report wurde hinzugefügt");
        }

        [Command("save"), Summary("Save Session")]
        public async Task ReportAsync([Remainder, Summary("Session Name")] string name)
        {
            if (name.Equals("?"))
            {
                await this.ReplyAsync($"Gespeicherte Sessions:");
                await this.ReplyAsync(this.ListSessions());
            }
        }

        private string[] ListSessions()
        {
            return Directory.GetDirectories(@"..\..\sessions");
        }


    }
}
