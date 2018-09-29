using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Inventory
    {
        private int Id { get; set; }
        private List<string> Items { get; set; } = new List<string>();
        private List<string> Food { get; set; } = new List<string>();
        private List<Weapon> Weapons { get; set; } = new List<Weapon>();
    }
}
