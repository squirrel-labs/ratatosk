using System;

namespace DSALib.Models.Database.Dsa {
    public class Advantage {
        public Advantage(string name, string value = "") {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            Value = value ?? throw new ArgumentNullException(nameof(value));
        }

        public string Name { get; set; }
        public string Value { get; set; }
    }
}