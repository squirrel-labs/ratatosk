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

        public Group(string name, int userOnline)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
        }

        public string Name { get; set; }
        public string Password { get; set; }
        public List<User> Users { get; set; } = new List<User>();

        public int UserCount
        {
            get { return Users.Count; }
        }

        public SendGroup SendGroup()
        {
            return new SendGroup( Name, UserCount);
        }
    }

    public class SendGroup
    {
        public SendGroup(string name, int userCount)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            UserCount = userCount;
        }

        public string Name { get; set; }

        public int UserCount { get; set; }
        
    }
}
