using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Talent
    {
        private string Name { get; set; }

        private Roll Roll { get; set; } = new Roll();
    }
}
