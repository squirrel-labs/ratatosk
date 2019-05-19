using System;

namespace DSALib.Models.Database.DSA
{
    public class Talent : DSALib.Models.Database.DataObject
    {
        public Talent()
        {
        }

        public Talent(string name)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
        }

        public Talent(string name, string roll)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Roll = roll.Split('/');
        }

        public string[] Roll { get; set; } = new string[3];
    }
}