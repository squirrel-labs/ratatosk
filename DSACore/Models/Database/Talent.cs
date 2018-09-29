using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Talent
    {
        public string Name { get; set; }

        public Roll Roll { get; set; } = new Roll();
    }
}
