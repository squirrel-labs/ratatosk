using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.Collections;
    using System.IO;
    using System.Reflection;
    using System.Runtime.CompilerServices;

    using DiscoBot.Audio;
    using DiscoBot.Auxiliary;
    using DiscoBot.Commands;

    using Discord;

    using Newtonsoft.Json;

    public static class Properties
    {
        private static Dictionary<string, object> objects;

        static Properties()
        {
            objects = new Dictionary<string, object>();
            /*this.objects.Add("Sounds", new List<Sound>());
            this.objects.Add("CommandInfos", new List<CommandInfo>());*/
        }

        public static List<CommandInfo> CommandInfos { get => objects["CommandInfo"] as List<CommandInfo>; set => objects["CommandInfo"] = value; } // use Properties.Commandinfos to access the abstract Object array

        public static List<Sound> Sounds { get => objects["Sound"] as List<Sound>; set => objects["Sound"] = value; }

        public static void Deserialize(string path = @"..\..\sessions")
        {
            
                var files = Directory.GetFiles(path, "*.json");

                foreach (string file in files)
                {
                    try
                    {
                        string name = file.Split('\\').Last().Split('.')[0].Replace('-', '.');
                        string data = File.ReadAllText(file);
                        Type type = Type.GetType(name);
                        if (data.StartsWith("["))
                        {
                            type = typeof(List<>).MakeGenericType(type);
                        }

                        var o = JsonConvert.DeserializeObject(data, type);
                        objects.Add(name.Split('.').Last(), o);
                    }
                    catch (Exception e)
                    {
                        // ignored
                        var log = new LogMessage(LogSeverity.Warning, "Properties", $"Laden von Save-File {file} fehlgeschlagen.", e);
                        Console.WriteLine(log);
                    }

                }
            
        }

        public static void Serialize(string path = @"..\..\sessions\")
        {
            try
            {
                foreach (var o in objects)
                {
                    string assembly = o.Value is IList list ? ((IList)list)[0]?.GetType().FullName : o.Value.GetType().FullName;

                    var name = path + assembly.Replace('.', '-') + ".json";
                    File.WriteAllText(name, JsonConvert.SerializeObject(o.Value, Formatting.Indented)); // Deserialize Data and create CommandInfo Struct
                }
            }
            catch (Exception e)
            {
                // ignored
            }
        }
    }
}
