using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Talent
    {
        public Talent(string name, String roll)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Roll = roll.Split('/');
        }

        public string Name { get; set; }

        public string[] Roll { get; set; } = new string[3];
    }
}
