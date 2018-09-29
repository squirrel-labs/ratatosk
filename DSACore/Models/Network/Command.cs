using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Network
{
    public class Command
    {
        public ulong GroupId { get; set; } = 0;
        public ulong CharId { get; set; }
        public string Name { get; set; }
        public string CmdIdentifier { get; set; }
        public List<string> CmdTexts { get; set; }
        public string CmdText => CmdTexts.Count != 0 ? CmdTexts.First() : "";

        public int Cmdmodifier => CmdTexts.Count != 0 && int.TryParse(CmdTexts.Last(), out int mod) ? mod : 0;
        public bool IsDm { get; set; } = false;
    }
}
