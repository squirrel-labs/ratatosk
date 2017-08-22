using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml;

namespace DiscoBot
{
    public class Char
    {

        public string name;
        public Dictionary<string, int> eigenschaften = new Dictionary<string, int>();
        public List<Talent> talente = new List<Talent>();
        public List<Kampf> kampftalente = new List<Kampf>();

        public Dictionary<string, string> Proptable = new Dictionary<string, string>();


        public Char(String path )
        {

            Load(path);
        }

        private void Load(string path)
        {
            XmlTextReader reader = new XmlTextReader(path);
            while (reader.Read())
            {
                if (reader.NodeType == XmlNodeType.Element)
                    switch (reader.Name)
                    {
                        case "held":
                            name = reader.GetAttribute("name");
                            break;
                        case "eigenschaft":
                            eigenschaften.Add(reader.GetAttribute("name"), Convert.ToInt32(reader.GetAttribute("value")) + Convert.ToInt32(reader.GetAttribute("mod")));
                            break;
                        case "talentliste":
                            reader.Read();
                            while (reader.Name.Equals("talent"))
                            {
                                talente.Add(new Talent(reader.GetAttribute("name"), reader.GetAttribute("probe").Remove(0, 2).Trim(')'), Convert.ToInt32(reader.GetAttribute("value"))));
                                reader.Read();
                            }
                            break;
                        case "kampfwerte":
                            string atname = reader.GetAttribute("name");
                            reader.Read();
                            int at = Convert.ToInt32(reader.GetAttribute("value"));
                            reader.Read();
                            int pa = Convert.ToInt32(reader.GetAttribute("value"));
                            kampftalente.Add(new Kampf(atname, at, pa));
                            break;
                    }



            }
            Proptable.Add("MU", "Mut");
            Proptable.Add("KL", "Klugheit");
            Proptable.Add("IN", "Intuition");
            Proptable.Add("CH", "Charisma");
            Proptable.Add("FF", "Fingerfertigkeit");
            Proptable.Add("GE", "Gewandtheit");
            Proptable.Add("KO", "Konstitution");
            Proptable.Add("KK", "Körperkraft");

        }
        public string TestTalent(string talent)
        {
            var output = new StringBuilder();
            var ttalent = talente.Find(v => v.name.Equals(talent));
            var props = ttalent.Test();
            int tap = ttalent.value;
            for (int i = 0; i <= 2; i++)
            {
                int temp = dice.Rolld20();
                int eigenschaft = eigenschaften[Proptable[props[i]]];
                if (eigenschaft < temp)
                    tap -= temp - eigenschaft;
                output.Append(temp + " ");
            }
            output.Append("tap: " + tap);
            return output.ToString();
        }
        public string Angriff(string talent)
        {
            var attack = kampftalente.Find(x => x.name.Equals(talent));
            int tap = attack.at/*+eigenschaften["at"]*/;
            int temp = dice.Rolld20();
            tap -= temp;
            return temp + " " + tap;
        }
        public string Parade(string talent)
        {
            var attack = kampftalente.Find(x => x.name.Equals(talent));
            int tap = attack.pa /*+ eigenschaften["pa"]*/;
            int temp = dice.Rolld20();
            tap -= temp;
            return temp + " " + tap;
        }
        public string Fernkampf(string talent,int erschwernis=0)
        {
            var attack = talente.Find(v => v.name.Equals(talent));
            int tap = attack.value + eigenschaften["fk"]-erschwernis;
            int temp = dice.Rolld20();
            tap -= -temp;
            return temp + " " + tap;
        }

    }
    public class Talent
    {
        public string name, probe;
        public int value;
        public Talent(string name, string probe, int value) { this.name = name; this.probe = probe; this.value = value; }
        public string[] Test()
        {
            var temp = probe.Split('/');
            foreach (string s in temp)
                s.Replace("/", "");
            return temp;
        }

    }
    public class Kampf
    {
        public string name;
        public int at, pa;
        public Kampf(string name, int at, int pa) { this.name = name; this.at = at; this.pa = pa; }
        void Test() { }
    }
    public static class dice
    {
        static System.Random rnd = new System.Random();
        public static int Rolld20()
        {
            return rnd.Next(1, 21);
        }
    }
}
