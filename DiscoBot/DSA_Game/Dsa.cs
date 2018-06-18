namespace DiscoBot.DSA_Game
{
    using System;
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;

    using DiscoBot.Audio;
    using DiscoBot.Commands;
    using DiscoBot.DSA_Game.Characters;
    using DiscoBot.DSA_Game.Save;

    using Discord.Commands;

    public static class Dsa
    {
        public static ICommandContext GeneralContext { get; set; }

        public static AudioService Service { get; set; }

        public static List<ICharacter> Chars { get; set; } = new List<ICharacter>();  // list of all characters

        public static List<Talent> Talente { get; set; } = new List<Talent>();

        public static Session Session { get; set; }

        public static void Startup()
        {

            // relation.Add("Papo", "Pump aus der Gosse");
            foreach (var filename in Directory.GetFiles("helden", "*.xml"))
            {
                Chars.Add(new Character(filename));
                (Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Talente.Add(v));
            }

            Properties.Deserialize();
            Properties.Serialize();

            Talente = Talente.OrderBy(x => x.Name).ToList();

            Session = new Session
                          {
                              Chars = Chars.Select(x => SaveChar.FromICharacter(x)).ToList(),
                              GeneralContext = GeneralContext
                          };
            Session.Save();
        }
    }
}