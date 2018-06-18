using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.IO;

    using DiscoBot.DSA_Game.Characters;

    using Discord.Commands;

    using Newtonsoft.Json;

    public class Session
    {
        public ICommandContext GeneralContext { get; set; }

        public Dictionary<string, string> Relation { get; set; } = new Dictionary<string, string>(); // dictionary to match the char

        public List<ICharacter> Chars { get; set; } = new List<ICharacter>();  // list of all characters

        public string SessionName { get; set; }

        public static Session Load(string path = @"..\..\session.json")
        {
            try
            {
                return JsonConvert.DeserializeObject<Session>(File.ReadAllText(path)); // Deserialize Data and create Session Object
            }
            catch (Exception e)
            {
                // ignored
                return null;
            }
        }

        public void Save(string path = @"..\..\session.json")
        {
            try
            {
                File.WriteAllText(path, JsonConvert.SerializeObject(this, Formatting.Indented)); // Deserialize Data and create CommandInfo Struct
            }
            catch (Exception e)
            {
                // ignored
            }
        }
    }
}
