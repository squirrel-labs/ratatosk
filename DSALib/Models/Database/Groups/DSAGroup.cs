using System.Collections.Generic;
using DSALib.Models.Database.Dsa;

namespace DSALib.Models.Database.Groups
{
    public class DsaGroup : Group
    {
        public List<GroupChar> Chars { get; set; } = new List<GroupChar>();
    }
}