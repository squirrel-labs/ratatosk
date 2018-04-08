namespace DiscoBot
{
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;

    using Discord.Commands;

    public static class DSA
    {
        public static ICommandContext GeneralContext { get; set; }
        public static Dictionary<string, string> relation = new Dictionary<string, string>(); //dictionary to match the char
        public static List<ICharacter> chars = new List<ICharacter>();                                      //list of all charackters

        public static List<Talent> Talente { get; set; } = new List<Talent>();

        public static void Startup()
        {
            relation.Add("The Doctor", "Numeri Illuminus");//Relation
            relation.Add("Tardis", "Morla");//"Numeri Illuminus");
            relation.Add("DSA Bot", "Morla");//"Felis Exodus Schattenwald");
            relation.Add("Morla", "Morla");
            relation.Add("Rhoktar", "Rhoktar4");
            //relation.Add("Papo","Gwendelson");
            relation.Add("Papo", "Pump aus der Gosse");
            relation.Add("Potus", "Potus");
            //relation.Add("Papo", "Pump aus der Gosse");
            foreach (var filename in Directory.GetFiles("helden", "*.xml"))
            {
                chars.Add(new Character(filename));
                (chars.Last() as Character)?.Talente.Select(x => new Talent(x.name, x.probe, 0))
                    .Where(c => !Talente.Exists(v => v.name.Equals(c.name))).ToList().ForEach(v => Talente.Add(v));
            }

            Talente = Talente.OrderBy(x => x.name).ToList();
        }
    }
}