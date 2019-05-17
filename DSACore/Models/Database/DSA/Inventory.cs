using System.Collections.Generic;

namespace DSACore.Models.Database.DSA
{
    public class Inventory
    {
        public int Id { get; set; }
        public Dictionary<string, bool> Items { get; set; } = new Dictionary<string, bool>();
        public Dictionary<string, bool> Food { get; set; } = new Dictionary<string, bool>();
        public List<Weapon> Weapons { get; set; } = new List<Weapon>();
    }
}
