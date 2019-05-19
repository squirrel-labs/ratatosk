using DiscoBot.DSA_Game.Characters;

namespace DSALib.Characters
{
    public class Being : Entity
    {
        public int Lebenspunkte_Basis { get; set; } = 30;

        public int Lebenspunkte_Aktuell { get; set; } = 30;

        public int Ausdauer_Basis { get; set; } = 30;

        public int Ausdauer_Aktuell { get; set; } = 30;

        public int Astralpunkte_Basis { get; set; } = 0;

        public int Astralpunkte_Aktuell { get; set; } = 0;
    }
}