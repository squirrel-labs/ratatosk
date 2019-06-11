using System;
using System.Collections.Generic;
using System.Linq;
using DSALib.DSA_Game.Characters;
using DSALib.DSA_Game.Save;
using DSALib;
using DSALib.Characters;
using DSALib.Models.Dsa;

namespace DSALib.DSA_Game
{
    public static class Dsa
    {
#if DEBUG
        public const string
            rootPath = ""; //"C:\\Users\\Dennis\\Source\\Repos\\DiscoBot\\DSALib\\";//"DiscoBot\\DSALib\\";
#else
        public const string rootPath = "";//"DiscoBot\\DSALib\\";
#endif
        private static Session s_session;

        public static List<ICharacter> Chars { get; set; } = new List<ICharacter>(); // list of all characters

        public static List<Talent> Talente { get; set; } = new List<Talent>();

        public static List<Zauber> Zauber { get; set; } = new List<Zauber>();

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
                foreach (var x in value.Chars) Chars.Find(c => c.Name.Equals(x.Name)).Update(x);
            }
        }

        public static void Startup()
        {
            //new .Auxiliary.Calculator.StringSolver("1d100 - (1d200 + 1) * -50000").Solve();
            /*Session = new Session();*/
            // relation.Add("Papo", "Pump aus der Gosse");
            /*foreach (var filename in Directory.GetFiles(rootPath + "helden", "*.xml"))
            {
                Chars.Add(new Character(filename));
                (Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Talente.Add(v));
                (Chars.Last() as Character)?.Zauber.Select(x => new Zauber(x.Name, x.Probe, 0, x.Complexity))
                    .Where(c => !Zauber.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Zauber.Add(v));
            }
*/

            Properties.Deserialize();
            Properties.Serialize(rootPath + "Properties");


            Talente = Talente.OrderBy(x => x.Name).ToList();
            Zauber = Zauber.OrderBy(x => x.Name).ToList();

            /*foreach (var talent in Talente)
            {
                Database.AddTalent(new Models.Database.Talent(talent.Name, talent.Probe));
            }

            foreach (var talent in Zauber)
            {
                Database.AddSpell(new Models.Database.GeneralSpell(talent.Name, talent.Probe, talent.Complexity));
            }*/

            //new WeaponImporter().DownloadWeapons().Wait();


            Session = new Session
            {
                Chars = Chars.Select(SaveChar.FromICharacter).ToList()
            };
            //Session.Save();
        }

        public static ICharacter GetCharacter(ulong id)
        {
            throw new NotImplementedException();
        }

        public static ICharacter GetCharacter(string name, ulong groupId)
        {
            throw new NotImplementedException();
        }
    }
}