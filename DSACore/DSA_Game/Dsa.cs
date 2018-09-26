using DSALib;
using DSALib.Characters;

namespace DiscoBot.DSA_Game
{
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;
    using DiscoBot.DSA_Game.Characters;
    using DiscoBot.DSA_Game.Save;

    public static class Dsa
    {
        private static Session s_session;

        public static List<ICharacter> Chars { get; set; } = new List<ICharacter>();  // list of all characters

        public static List<Talent> Talente { get; set; } = new List<Talent>();

        public static Session Session
        {
            get
            {
                s_session.Chars = Chars.Select(x => SaveChar.FromICharacter(x)).ToList();
                return s_session;
            }

            set
            {
                s_session = value;
                foreach (var x in value.Chars)
                {
                    Chars.Find(c => c.Name.Equals(x.Name)).Update(x);
                }
            }
        }

        public static void Startup()
        {
            //new DiscoBot.Auxiliary.Calculator.StringSolver("1d100 - (1d200 + 1) * -50000").Solve();
            /*Session = new Session();*/
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
                Chars = Chars.Select(x => SaveChar.FromICharacter(x)).ToList()
            };
            Session.Save();
        }
    }
}