using System;

namespace DSALib.Models.Database.Dsa
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