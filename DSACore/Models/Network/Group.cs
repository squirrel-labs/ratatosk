using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Network
{
    public class Group
    {
        public Group(string name, string password)
        {
            Name = name;
            Password = password;
        }

        public string Name { get; set; }
        public string Password { get; set; }
        public List<User> Users { get; set; } = new List<User>();
    }
}
