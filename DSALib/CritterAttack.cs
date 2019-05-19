namespace DSALib
{
    public class CritterAttack
    {
        public CritterAttack(string name, int at, string tp, string comment = "")
        {
            Name = name;
            At = at;
            Tp = tp;
            Comment = comment;
        }

        public string Name { get; set; }

        public int At { get; set; }

        public string Tp { get; set; }

        public string Comment { get; set; }
    }
}