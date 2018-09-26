using System;
using System.Linq;
using System.Threading.Tasks;

namespace DiscoBot.DSA_Game.Save
{
    using System.IO;

    public class SaveCommand 
    {
        public void LoadSession(string name = "")
        {
            if (name.Equals("?") || name.Equals(string.Empty))
            {
                Console.WriteLine($"Gespeicherte Sessions:");
                Console.WriteLine(this.ListSessions());
                return;
            }

            var path = DSA_Game.Save.Session.DirectoryPath + @"\" + name;

            var files = Directory.GetFiles(path);
            var session = files.OrderByDescending(x => Convert.ToInt32(x.Split('-').Last().Split('.').First())).First();
            Dsa.Session = Session.Load(session);

            Console.WriteLine($"{name} wurde geladen");
        }

        public void SessionSave(string name = "")
        {
            //var sendFile = this.Context.Channel.SendWebFile("https://cdn.discordapp.com/attachments/377123019673567232/465615882048110603/giphy.gif");

            if (name.Equals("?") || name.Equals(string.Empty))
            {
                Console.WriteLine($"Gespeicherte Sessions:");
                Console.WriteLine(this.ListSessions());
                return;
            }

            var path = DSA_Game.Save.Session.DirectoryPath + @"\" + name;
            if (Directory.Exists(path))
            {
                var files = Directory.GetFiles(path);
                int current = files.Max(x => Convert.ToInt32(x.Split('-').Last().Split('.').First()));
                Dsa.Session.SessionName = name;
                Dsa.Session.Save(path + "\\" + name + $"-{++current}.json");
            }
            else
            {
                Directory.CreateDirectory(path);
                Dsa.Session.SessionName = name;
                Dsa.Session.Save(path + "\\" + name + $"-0.json");
            }

            Console.WriteLine($"{name} wurde gespeichert");
            //await sendFile;
        }

        private string[] ListSessions()
        {
            string[] dirs = Directory.GetDirectories(Session.DirectoryPath).OrderByDescending(x => new DirectoryInfo(x).LastAccessTime.Ticks).ToArray();
            for (int i = 0; i < dirs.Length; i++)
            {
                dirs[i] += "; " + new DirectoryInfo(dirs[i]).LastAccessTime;
            }

            return dirs;
        }
    }
}
