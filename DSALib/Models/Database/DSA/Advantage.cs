using System;

namespace DSACore.Models.Database.DSA
{
    public class Advantage
    {
        public Advantage(string name, string value = "")
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Value = value ?? throw new ArgumentNullException(nameof(value));
        }

        public string Name { get; set; }
        public string Value { get; set; }
    }
}