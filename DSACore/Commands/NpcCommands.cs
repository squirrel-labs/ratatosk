using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using DSACore.Characters;
using DSACore.DSA_Game;
using DSACore.DSA_Game.Characters;
using DSACore.Auxiliary;
using Microsoft.AspNetCore.Mvc.Internal;

namespace DSACore.Commands
{
    public class NpcCommands
    {
        public static string CreateNpc(ulong id, IEnumerable<string> props, int modifier)
        {
            if (int.TryParse(props.Last(), out var mean)) return Random(id, props.First(), mean, modifier);

            return Copy(id, props.First(), props.Last(), modifier);
        }

        private static string Random(ulong id, string npcName, int mean = 9, int stDv = 1)
        {
            throw new NotImplementedException();
            Dsa.Chars.Add(new Npc(npcName, mean, stDv));
            return $"{npcName} wurde zufällig generiert";
        }

        private static string Copy(ulong id, string npcName, string source, int stDv = 1)
        {
            if (Dsa.Chars.Exists(x => x.Name.Equals(npcName))) throw new Exception("Char gibt es schon");
            throw new NotImplementedException();
            var chr = Dsa.GetCharacter(id);
            Dsa.Chars.Add(new Character(chr as Character, npcName, stDv));
            return $"{npcName} wurde als variierte Kopie von {source} erstellt";
        }
    }
}