using System;
using System.Collections.Generic;
using System.Linq;
using DSALib.DSA_Game.Characters;

namespace DSALib.Models.Database.Dsa
{
    public class DatabaseChar
    {
        public DatabaseChar()
        {
        }

        public DatabaseChar(int id, string name, string rasse, List<Field> skills, List<Field> talents,
            List<Advantage> advantages, List<CharSpell> spells, List<WeaponTalent> weaponTalents)
        {
            Id = id;
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Race = rasse ?? throw new ArgumentNullException(nameof(rasse));
            Skills = skills ?? throw new ArgumentNullException(nameof(skills));
            Talents = talents ?? throw new ArgumentNullException(nameof(talents));
            Advantages = advantages ?? throw new ArgumentNullException(nameof(advantages));
            Spells = spells ?? throw new ArgumentNullException(nameof(spells));
            WeaponTalents = weaponTalents ?? throw new ArgumentNullException(nameof(weaponTalents));
        }

        public int Id { get; set; }

        public string Name { get; set; }

        public string Race { get; set; }

        public List<Field> Skills { get; set; } = new List<Field>();

        public List<Field> Talents { get; set; } = new List<Field>();

        public List<Advantage> Advantages { get; set; } = new List<Advantage>();

        public List<CharSpell> Spells { get; set; } = new List<CharSpell>();

        public List<WeaponTalent> WeaponTalents { get; set; } = new List<WeaponTalent>();


        public static void LoadChar(Character file, out GroupChar group, out DatabaseChar data)
        {
            group = new GroupChar();
            data = new DatabaseChar();

            group.Name = file.Name.Split(' ').First();
            group.Weapon = new Weapon();
            group.Lp = group.LpMax = file.Lebenspunkte_Basis;
            group.As = group.AsMax = file.Astralpunkte_Basis;
            group.Weapon = new Weapon();

            data.Name = file.Name;
            data.Advantages = file.Vorteile.Select(x => new Advantage(x.Name, x.Value)).ToList();
            data.Skills = file.Eigenschaften.Select(x => new Field(x.Key, x.Value)).ToList();
            data.Spells = file.Zauber.Select(x => new CharSpell(x.Representation, x.Value)).ToList();
            data.Talents = file.Talente.Select(x => new Field(x.Name, x.Value)).ToList();
            data.WeaponTalents = file.Kampftalente.Select(x => new WeaponTalent(x.Name, x.At, x.Pa)).ToList();
        }
    }
}