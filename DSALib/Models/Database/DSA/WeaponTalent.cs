using System;

namespace DSALib.Models.Database.DSA
{
    public class WeaponTalent
    {
        public WeaponTalent(string name, int at, int pa)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            At = at;
            Pa = pa;
        }

        public string Name { get; set; }
        public int At { get; set; }
        public int Pa { get; set; }
    }
}