namespace DiscoBot.Commands
{
    using System;
    using System.Collections.Generic;
    using System.Linq;
    using System.Text;
    using System.Threading.Tasks;

    using DiscoBot.Audio;
    using DiscoBot.Auxiliary;
    using DiscoBot.DSA_Game;
    using DiscoBot.DSA_Game.Characters;

    using Discord.Commands;

    public class List : ModuleBase
    {
        [Command("list"), Summary("gibt eine Auflistung  aus")]
        public async Task ListAsync([Summary("Aktion")] string prop)
        {
            var res = new List<string>();
            
            int persist = 0;

            switch (prop.ToLower())
            {
                case "man":
                case "help":
                    await this.ReplyAsync("```xl\n" + Help.Get_Specific_Help("List") + "\n```");
                    return;
                   // break;
                case "chars":
                    res.AddRange(Dsa.Chars.Select(x => x.Name));
                    break;
                case "commands":
                    // res.AddRange(Help.Commands.Select(x => x.Name));
                    res.Add(Help.Get_Generic_Help());
                    break;
                case "play":
                case "sound":
                case "sounds":
                    res.AddRange(
                        Enum.GetNames(typeof(Sound)));
                    break;

                default:
                    res.Add($"Kommando {prop} nicht gefunden");
                    break;
            }
            

            if (persist == 1)
            {
                await this.ReplyAsync(res);
            }
            else
            {
                await this.ReplyAsync(res, TimeSpan.FromSeconds(90));
            }
        }
    }
}
