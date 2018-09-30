using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Database
{
    public class Group
    {
        public string Name { get; set; }
        public string Discord { get; set; }
        public string Password { get; set; }
        public int Id { get; set; }
        public List<GroupChar> Chars { get; set; }= new List<GroupChar>();
    }
}
