namespace DSALib
{
    public class KampfTalent
    {
        public KampfTalent(string name, int at, int pa)
        {
            this.Name = name;
            this.At = at;
            this.Pa = pa;
        }

        public string Name { get; set; }

        public int At { get; set; }

        public int Pa { get; set; }
    }
}
