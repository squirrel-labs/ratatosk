using System;

namespace DSACore.Models.Database.DSA
{
    public class Talent
    {
        public Talent()
        {
        }

        public Talent(string name)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
        }

        public Talent(string name, String roll)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Roll = roll.Split('/');
        }

        public string Name { get; set; }

        public string[] Roll { get; set; } = new string[3];
    }
}
