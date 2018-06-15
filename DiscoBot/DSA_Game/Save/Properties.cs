using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.IO;

    using DiscoBot.Audio;
    using DiscoBot.Auxiliary;
    using DiscoBot.Commands;

    using Newtonsoft.Json;

    public class Properties
    {
        public List<CommandInfo> CommandInfos { get; set; }

        public List<Sound> Sounds { get; set; }

        public static Properties Deserialize(string path = @"..\..\Properties.json")
        {   
            try
            {
                return JsonConvert.DeserializeObject<Properties>(File.ReadAllText(path)); // Deserialize Data and create CommandInfo Struct
            }
            catch (Exception e)
            {
                // ignored
                return null;
            }
        }

        public void Serialize(string path = @"..\..\Properties.json")
        {/*
            var stream = new StreamWriter(path); // Load properties file
            var reader = new JsonTextWriter(stream); // create stream reader*/

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
