namespace DSALib
{
    public class KampfTalent
    {
        public KampfTalent(string name, int at, int pa)
        {
            Name = name;
            At = at;
            Pa = pa;
        }

        public string Name { get; set; }

        public int At { get; set; }

        public int Pa { get; set; }
    }
}