using System;
using System.Collections.Generic;

namespace DSACore.DSA_Game.Save
{
    using System.IO;
    using Characters;
    using Newtonsoft.Json;

    public class Session
    {
        public static string DirectoryPath { get; set; } = @"sessions";

        public Dictionary<string, string> Relation { get; set; } = new Dictionary<string, string>(); // dictionary to match the char

        public List<SaveChar> Chars { get; set; } = new List<SaveChar>();  // list of all characters

        public string SessionName { get; set; }
        
        public static Session Load(string path)
        {
            try
            {
                return JsonConvert.DeserializeObject<Session>(File.ReadAllText(path)); // Deserialize Data and create Session Object
            }
            catch (Exception e)
            {
                // ignored
                Console.WriteLine($"Laden von Save-File {path} fehlgeschlagen."+ e); 
                return null;
            }
        }

        public void Save(string path)
        {
            try
            {
                File.WriteAllText(path, JsonConvert.SerializeObject(this, Formatting.Indented)); // Deserialize Data and create CommandInfo Struct
            }
            catch (Exception e)
            {
                Console.WriteLine($"Speichern von Save-File {path} fehlgeschlagen.\n"+ e);
                // ignored
            }
        }
    }
}
