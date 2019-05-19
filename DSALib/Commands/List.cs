using System;
using System.Collections.Generic;
using System.Linq;
using DSALib.DSA_Game;

namespace DSALib.Commands
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

                default:
                    res.Add($"Kommando {prop} nicht gefunden");
                    break;
            }


            return res.ToString();
        }
    }
}