using System;

namespace DSACore.Models.Database.DSA
{
    public class Weapon
    {
        public Weapon()
        {
        }

        public Weapon(string name, string damage, int weight, string weaponTalent, string price)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Damage = damage ?? throw new ArgumentNullException(nameof(damage));
            Weight = weight;
            WeaponTalent = weaponTalent ?? throw new ArgumentNullException(nameof(weaponTalent));
            Price = price;
        }

        public string Name { get; set; }
        public string Damage { get; set; }
        public int Weight { get; set; }
        public string WeaponTalent { get; set; }
        public string Price { get; set; }
    }

    public class MeleeWeapon : Weapon
    {
        public string TpKK { get; set; }
        public int INI { get; set; }
        public string MW { get; set; }

        public MeleeWeapon(string name, string damage, int weight, string weaponTalent, string price) : base(name,
            damage, weight, weaponTalent, price)
        {
        }
    }

    public class RangedWeapon : Weapon
    {
        public int AtMod { get; set; }
        public int KKMod { get; set; }
        public string AtReach { get; set; }
        public string TpReach { get; set; }
        public int LoadTime { get; set; }

        public RangedWeapon(string name, string damage, int weight, string weaponTalent, string price) : base(name,
            damage, weight, weaponTalent, price)
        {
        }
    }
}