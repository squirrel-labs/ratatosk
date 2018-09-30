using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class GeneralSpell : Talent
    {
        public char Comlexity = 'A';

        public GeneralSpell(string name, string roll, char comlexity = 'A') :base(name, roll)
        {
            Comlexity = comlexity;
        }

        public GeneralSpell(string name, string roll) : base(name, roll)
        {
        }
    }
}
