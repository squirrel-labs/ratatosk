namespace DSACore.Models.Database.DSA
{
    public class GeneralSpell : Talent
    {
        public char Comlexity = 'A';

        public GeneralSpell(string name, string roll, char comlexity = 'A') : base(name, roll)
        {
            Comlexity = comlexity;
        }

        public GeneralSpell(string name, string roll) : base(name, roll)
        {
        }

        public GeneralSpell()
        {
        }
    }
}