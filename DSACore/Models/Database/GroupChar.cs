using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class GroupChar
    {
        public string Name { get; set; }
        public int Id { get; set; }
        public int Lp { get; set; }
        public int LpMax { get; set; }
        public int As { get; set; }
        public int AsMax { get; set; }
        public Weapon Weapon { get; set; }
    }
}
