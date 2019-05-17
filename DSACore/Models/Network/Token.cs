using System;
using Microsoft.EntityFrameworkCore;

namespace DSACore.Models.Network
{
    public class Token
    {
        public string Group { get; set; }
        private DateTime creation = DateTime.Now;

        public Token(string @group)
        {
            Group = @group;
        }

        public bool IsValid()
        {
            return DateTime.Now - creation < TimeSpan.FromMinutes(1);
        }
    }
}