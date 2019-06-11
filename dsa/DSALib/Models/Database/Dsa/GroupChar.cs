namespace DSALib.Models.Database.Dsa
{
    public class GroupChar
    {
        public string Name { get; set; }
        public int Id { get; set; }
        public int Lp { get; set; }
        public int LpMax { get; set; }
        public int As { get; set; }
        public int AsMax { get; set; }
        public Weapon Weapon { get; set; }
    }
}