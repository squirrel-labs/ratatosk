namespace DiscoBot.Auxiliary
{
    public class Vorteil // talent objekt
    {
        public Vorteil(string name, int value = 0)
        {
            this.Name = name;
            this.Value = value;
        }

        public string Name { get; set; }

        public int Value { get; set; }
    }
}
