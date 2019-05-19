using System;
using System.Collections.Generic;
using System.Linq;
using DSACore.Audio;
using DSACore.DSA_Game;

namespace DSACore.Commands
{
    public class List
    {
        public static string ListAsync(string prop)
        {
            var res = new List<string>();

            //int persist = 0;

            switch (prop.ToLower())
            {
                case "man":
                case "help":
                    return Help.Get_Specific_Help("List");
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


            return res.ToString();
        }
    }
}