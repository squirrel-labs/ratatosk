using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Characters
{
    using Discord;

    public class SaveChar
    {
        public string Name { get; set; }

        public int Lebenspunkte_Aktuell { get; set; }

        public int Ausdauer_Aktuell { get; set; }

        public int Astralpunkte_Aktuell { get; set; }

        public static SaveChar FromICharacter(ICharacter c)
        {
            return new SaveChar
            {
                Astralpunkte_Aktuell = c.Astralpunkte_Aktuell,
                Ausdauer_Aktuell = c.Ausdauer_Aktuell,
                Lebenspunkte_Aktuell = c.Lebenspunkte_Aktuell,
                Name = c.Name
            };
        }
    }


    public static class ICharExtension
    {
        public static void Update(this ICharacter c, SaveChar s)
        {
            c.Astralpunkte_Aktuell = s.Astralpunkte_Aktuell;
            c.Ausdauer_Aktuell = s.Ausdauer_Aktuell;
            c.Lebenspunkte_Aktuell = s.Lebenspunkte_Aktuell;
            c.Name = s.Name;
        }
    }
}
