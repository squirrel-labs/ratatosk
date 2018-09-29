using System;
using DSALib;
using DSALib.Characters;
using Microsoft.EntityFrameworkCore.Design;

namespace DSACore.DSA_Game
{
    using System.Collections.Generic;
    using System.IO;
    using System.Linq;
    using DSACore.DSA_Game.Characters;
    using DSACore.DSA_Game.Save;

    public static class Dsa
    {
        public const string rootPath = "";//"DiscoBot\\DSACore\\";

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

        public static void start(){}

        public static void Startup()
        {
            //new .Auxiliary.Calculator.StringSolver("1d100 - (1d200 + 1) * -50000").Solve();
            /*Session = new Session();*/
            // relation.Add("Papo", "Pump aus der Gosse");
            foreach (var filename in Directory.GetFiles(rootPath + "helden", "*.xml"))
            {
                Chars.Add(new Character(filename));
                (Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Talente.Add(v));
            }

            Properties.Deserialize(rootPath+"Properties");
            Properties.Serialize(rootPath + "Properties");

            Talente = Talente.OrderBy(x => x.Name).ToList();
            Session = new Session
            {
                Chars = Chars.Select(x => SaveChar.FromICharacter(x)).ToList()
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