namespace DiscoBot.Auxiliary
{
    public class Vorteil // talent objekt
    {
        public Vorteil(string name, string value = "")
        {
            this.Name = name;
            this.Value = value;
           // this.Choice = choice;
        }

        public string Name { get; set; }

        public string Value { get; set; }

        //public string Choice { get; set; }
    }
}
