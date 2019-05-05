using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using DSACore.Audio;
using DSACore.Auxiliary;
using Newtonsoft.Json;

namespace DSACore.DSA_Game.Save
{
    using System.Collections;
    using System.IO;
    using Newtonsoft.Json;

    public static class Properties
    {
        public static Dictionary<string, object> objects;

        static Properties()
        {
            objects = new Dictionary<string, object>();
            /*this.objects.Add("Sounds", new List<Sound>());
            this.objects.Add("CommandInfos", new List<CommandInfo>());*/
        }

        public static List<CommandInfo> CommandInfos
        {
            get => objects["CommandInfo"] as List<CommandInfo>;
            set => objects["CommandInfo"] = value;
        } // use Properties.Commandinfos to access the abstract Object array

        public static List<Sound> Sounds
        {
            get => objects["Sound"] as List<Sound>;
            set => objects["Sound"] = value;
        }

        public static void Deserialize(string path = @"Properties")
        {
            var files = Directory.GetFiles(path, "*.json");

            foreach (string file in files)
            {
                try
                {
                    string name = file.Split('\\').Last().Split('/').Last().Split('.')[0].Replace('-', '.');
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
                    Console.WriteLine($"Laden von Save-File {file} fehlgeschlagen." + e);
                }
            }
        }

        public static void Serialize(string path = @"..\..\Properties\")
        {
            try
            {
                foreach (var o in objects)
                {
                    string assembly = o.Value is IList list
                        ? ((IList) list)[0]?.GetType().FullName
                        : o.Value.GetType().FullName;

                    var name = path + assembly.Replace('.', '-') + ".json";
                    File.WriteAllText(name,
                        JsonConvert.SerializeObject(o.Value,
                            Formatting.Indented)); // Deserialize Data and create CommandInfo Struct
                }
            }
            catch (Exception e)
            {
                // ignored
                Console.WriteLine($"Speichern von Save-File fehlgeschlagen." + e);
            }
        }
    }
}