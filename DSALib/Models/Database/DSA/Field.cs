using System;

namespace DSALib.Models.Database.DSA
{
    public class Field
    {
        public Field(string name, int value = 0)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Value = value;
        }

        public string Name { get; set; }
        public int Value { get; set; }
    }
}