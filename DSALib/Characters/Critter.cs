using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DSALib.Characters
{
    using DiscoBot.DSA_Game.Characters;

    public class Critter : Being, ICombatant
    {
        private int rs, mr, ko, pa, gs, gw;
        

        public Critter(int gw, int gs, int rs, int mr, int ko, int pa)
        {
            this.gw = gw;
            this.gs = gs;
            this.rs = rs;
            this.mr = mr;
            this.ko = ko;
            this.pa = pa;
        }

        public string Angriff(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string Parade(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }
    }
}
