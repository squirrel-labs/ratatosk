namespace DiscoBot.DSA_Game
{
    using System;

    using DiscoBot.Auxiliary;

    public class Talent // talent objekt
    {
        public Talent(string name, string probe, int value)
        {
            this.Name = name;
            this.Probe = probe;
            this.Value = value;
        }

        public string Name { get; set; }

        public string Probe { get; set; }

        public int Value { get; set; }
        
        public string[] GetEigenschaften() // turn XX/XX/XX into string[]{XX,XX,XX}
        {
            var temp = this.Probe.Split('/');
            for (var index = 0; index < temp.Length; index++)
            {
                temp[index] = temp[index].Replace("/", string.Empty);
            }

            return temp;
        }

        public int CheckName(string quarry)
        {
            var sc = (StringComparer)new SpellCorrect();
            return sc.Compare(quarry, this.Name);
        }

        public bool IstFernkampftalent()
        {
            switch (Name)
            {
                case "Armbrust":
                case "Belagerungswaffen":
                case "Blasrohr":
                case "Bogen":
                case "Diskus":
                case "Schleuder":
                case "Wurfbeile":
                case "Wurfmesser":
                case "Wurfspeere":
                    return true;
                default:
                    return false;
            }
        }
    }
}
