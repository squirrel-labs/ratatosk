using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models
{
    public class Command
    {
        public ulong GroupId { get; set; } = 0;
        public ulong CharId { get; set; } 
        public string Name { get; set; }
        public string CmdIdentifier { get; set; }
        public List<string> CmdTexts { get; set; }
        public string CmdText => CmdTexts != null ? CmdTexts.First() : "";

        public int Cmdmodifier { get; set; } = 0;
        public bool IsDm { get; set; } = false;
    }
}
