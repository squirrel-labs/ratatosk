using System;

namespace DSALib.Models.Network
{
    public class Token
    {
        private readonly DateTime creation = DateTime.Now;

        public Token(string group)
        {
            Group = group;
        }

        public string Group { get; set; }

        public bool IsValid()
        {
            return DateTime.Now - creation < TimeSpan.FromMinutes(1);
        }
    }
}