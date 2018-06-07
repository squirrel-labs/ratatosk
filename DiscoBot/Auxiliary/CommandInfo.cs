using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.Auxiliary
{
    public struct CommandInfo
    {
        public CommandInfo(string name, string brief, string[] description, string scope)
        {
            this.Name = name;
            this.Scope = scope;
            this.Brief = brief;
            this.Description = description;
        }

        public string Name { get; }

        public string Scope { get; }

        public string Brief { get; }

        public string[] Description { get; }

        public string GetDescription()
        {
            return this.Description.Aggregate((s, c) => s + c);
        }
    }
}
