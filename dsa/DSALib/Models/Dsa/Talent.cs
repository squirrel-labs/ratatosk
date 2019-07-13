using DSALib.Models.Database;

namespace DSALib.Models.Dsa {
    public class Talent : DataObject // talent objekt
    {
        public Talent(string name, string probe, int value) {
            Name = name;
            Probe = probe;
            Value = value;
        }

        public string Probe { get; set; }

        public int Value { get; set; }

        public string[] GetEigenschaften() // turn XX/XX/XX into string[]{XX,XX,XX}
        {
            var temp = Probe.Split('/');
            for (var index = 0; index < temp.Length; index++) temp[index] = temp[index].Replace("/", string.Empty);

            return temp;
        }

        public bool IstFernkampftalent() {
            switch (Name) {
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