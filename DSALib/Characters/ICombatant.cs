using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Characters
{
    public interface ICombatant
    {
        string Name { get; set; }

        int Lebenspunkte_Basis { get; set; }
        int Lebenspunkte_Aktuell { get; set; }

        int Ausdauer_Basis { get; set; }
        int Ausdauer_Aktuell { get; set; }

        int Astralpunkte_Basis { get; set; }
        int Astralpunkte_Aktuell { get; set; }

        string Angriff(string talent, int erschwernis = 0);

        string Parade(string talent, int erschwernis = 0);
    }
}