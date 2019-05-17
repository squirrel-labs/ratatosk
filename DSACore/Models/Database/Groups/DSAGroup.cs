using System.Collections.Generic;
using DSACore.Models.Database.DSA;

namespace DSACore.Models.Database.Groups
{
    public class DSAGroup : Group
    {
        public List<GroupChar> Chars { get; set; }= new List<GroupChar>();
    }
}