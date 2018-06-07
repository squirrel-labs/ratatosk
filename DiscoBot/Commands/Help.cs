using System.Linq;

namespace DiscoBot.Commands
{
    using System;
    using System.Collections.Generic;
    using System.IO;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;

    using Discord.Commands;

    using Newtonsoft.Json;

    using CommandInfo = DiscoBot.Auxiliary.CommandInfo;

    public class Help : ModuleBase
    {
        static Help()
        {
            TextReader stream = new StreamReader(@"..\..\Help.json"); // Load command-description file
            var reader = new JsonTextReader(stream); // create stream reader

            reader.Read(); // step into structure, until the array starts
            reader.Read();
            reader.Read();
            
            try
            {
                var test = new JsonSerializer().Deserialize<List<CommandInfo>>(reader); // Deserialize Data and create CommandInfo Struct
                
                Commands.AddRange(test); // Add new CommandInfos to List
            }
            catch (Exception e)
            {
                // ignored
            }
        }

        public static List<CommandInfo> Commands { get; } = new List<CommandInfo>();


        public static string Get_Specific_Help(string command)
        {
            // return command specific help
            var com = Commands.OrderBy(x => SpellCorrect.CompareEasy(x.Name, command.ToLower())).First(); // get best fit command
            return com.GetDescription();
        }

        [Command("help"), Summary("prints the help menu.")]
        [Alias("Help", "man", "Man")]
        public async Task ShowHelpAsync(string command = "")
        {
            if (command.Equals(string.Empty)) // return generic Help
            {
                string res = "";

                foreach (var com in Commands)
                {
                    res += "!" + com.Name + ": " + com.Brief;

                    if (com.Description.Length > 1)
                    {
                        res += "\n\t(!man " + com.Name + " gibt genauere Informationen)";
                    }

                    res += "\n";
                }
                //await this.ReplyAsync("```\n[hilfreiche Erklärungen]\nAuflistung aller Commands mit !list commands\n```");
                await this.ReplyAsync("```xl\n" + res +"\n```");
                return;
            }



            // return command specific help
            //var com = Commands.OrderBy(x => SpellCorrect.CompareEasy(x.Name, command.ToLower())).First(); // get best fit command

            //await this.ReplyAsync("```xl\n" + com.GetDescription() + "\n```");
            await this.ReplyAsync("```xl\n" + Get_Specific_Help(command) + "\n```");
        }
    }
}
