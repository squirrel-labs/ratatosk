using System.Collections.Generic;
using DSALib.Models.Database.DSA;

namespace DSALib.Models.Database.Groups
{
    public class DSAGroup : Group
    {
        public List<GroupChar> Chars { get; set; } = new List<GroupChar>();
    }
}