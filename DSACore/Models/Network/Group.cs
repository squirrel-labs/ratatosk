using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Network
{
    public class Group
    {
        private int _online;

        public Group(string name, string password)
        {
            Name = name;
            Password = password;
        }

        public Group(string name, int userCount)
        {
            Name = name ?? throw new ArgumentNullException(nameof(name));
            UserCount = userCount;
        }

        public string Name { get; set; }
        public string Password { get; set; }
        public List<User> Users { get; set; } = new List<User>();

        public int UserCount
        {
            get { return _online; RefreshOnline();}
            set { _online = value; RefreshOnline();}
        }

        private void RefreshOnline()
        {
            _online = Users.Count;
        }

        public Group SendGroup()
        {
            return new Group( Name, UserCount);
        }
    }
}
