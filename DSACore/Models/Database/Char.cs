using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Char
    {
        private int Id { get; set; }

        private string Name { get; set; }

        private string Rasse { get; set; }

        private List<Field> Skills { get; set; } = new List<Field>();

        private List<Field> Talents { get; set; } = new List<Field>();

        private List<Field> Advantages { get; set; } = new List<Field>();

        private List<CharSpell> Spells { get; set; } = new List<CharSpell>();

        private List<WeaponTalent> WeaponTalents { get; set; } = new List<WeaponTalent>();

    }
}
