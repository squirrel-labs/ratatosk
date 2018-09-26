using DSACore.DSA_Game;
using DSACore.DSA_Game.Characters;

namespace DSACore.Commands
{
    using System;
    using System.Linq;
    using System.Net;

    using DSALib;

    public class FileHandler
    {
        public static string AddChar(ulong id, string url)
        {
            if (url == string.Empty)
            {
                throw new ArgumentException("Es wurde keine Datei angehängt");
            }
            

            if (!url.EndsWith(".xml"))
            {
                throw new ArgumentException("Es wurde kein xml Held mitgeschickt");
            }
            
                using (var client = new WebClient())
                {
                    client.DownloadFile(url, "helden\\" + url.Split("/").Last());
                }

                Dsa.Chars.Add(new Character("helden\\" + url.Split("/").Last()));
                (Dsa.Chars.Last() as Character)?.Talente.Select(x => new Talent(x.Name, x.Probe, 0))
                    .Where(c => !Dsa.Talente.Exists(v => v.Name.Equals(c.Name))).ToList().ForEach(v => Dsa.Talente.Add(v));

            return $"{url.Split("/").Last()} wurde erfolgreich gespeichert";
        }
    }
}