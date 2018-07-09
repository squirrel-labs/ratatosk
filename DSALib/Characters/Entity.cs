using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Characters
{
    public class Entity
    {
        public string Name { get; set; }

        public override string ToString()
        {
            return this.Name;
        }
    }
}
