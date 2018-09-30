using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class CharSpell
    {
        public CharSpell(string representation, int value)
        {
            this.representation = representation ?? throw new ArgumentNullException(nameof(representation));
            this.value = value;
        }

        public string representation { get; set; }
        public int value { get; set; }
    }
}
