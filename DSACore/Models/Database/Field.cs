using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Field
    {
        public Field(string name, int value = 0)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            this.Value = value;
        }

        public string Name { get; set; }
        public int Value { get; set; }
    }
}
