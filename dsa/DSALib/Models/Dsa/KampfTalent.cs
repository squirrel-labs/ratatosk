namespace DSALib.Models.Dsa
{
    public class KampfTalent : Database.DataObject
    {
        public KampfTalent(string name, int at, int pa)
        {
            Name = name;
            At = at;
            Pa = pa;
        }

        public int At { get; set; }

        public int Pa { get; set; }
    }
}