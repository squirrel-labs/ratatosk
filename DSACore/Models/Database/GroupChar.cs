using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class GroupChar
    {
        private string Name { get; set; }
        private int Id { get; set; }
        private int Lp { get; set; }
        private int LpMax { get; set; }
        private int As { get; set; }
        private int AsMax { get; set; }
        private Weapon Weapon { get; set; }
    }
}
