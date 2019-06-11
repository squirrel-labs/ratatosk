namespace DSALib.Models.Dsa
{
    public class Zauber : Talent
    {
        public Zauber(string name, string probe, int value, char complexity = 'A', string representation = "Magier")
            : base(name, probe, value)
        {
            Complexity = complexity;
            Representation = Representation;
        }

        public char Complexity { get; }

        public string Representation { get; }
    }
}