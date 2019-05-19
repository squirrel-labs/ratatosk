namespace DSALib
{
    public class Vorteil // talent objekt
    {
        public Vorteil(string name, string value = "")
        {
            Name = name;
            Value = value;
            // this.Choice = choice;
        }

        public string Name { get; set; }

        public string Value { get; set; }

        //public string Choice { get; set; }
    }
}