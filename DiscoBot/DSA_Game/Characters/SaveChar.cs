using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Characters
{
    public class SaveChar : ICharacter
    {
        public string Name { get; set; }

        public int Lebenspunkte_Basis { get; set; }

        public int Lebenspunkte_Aktuell { get; set; }

        public int Ausdauer_Basis { get; set; }

        public int Ausdauer_Aktuell { get; set; }

        public int Astralpunkte_Basis { get; set; }

        public int Astralpunkte_Aktuell { get; set; }

        public static SaveChar FromICharacter(ICharacter c)
        {
            return new SaveChar
            {
                Astralpunkte_Aktuell = c.Astralpunkte_Aktuell,
                Astralpunkte_Basis = c.Astralpunkte_Basis,
                Ausdauer_Aktuell = c.Ausdauer_Aktuell,
                Ausdauer_Basis = c.Ausdauer_Basis,
                Lebenspunkte_Aktuell = c.Lebenspunkte_Aktuell,
                Lebenspunkte_Basis = c.Lebenspunkte_Basis,
                Name = c.Name
            };
        }

        public string TestTalent(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string Angriff(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string Parade(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string Fernkampf(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string TestZauber(string waffe, int erschwernis)
        {
            throw new NotImplementedException();
        }
    }
}
