using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml;

namespace DiscoBot
{
    public class Character : ICharacter
    {
        public string Name { get; set; } //charname
        public Dictionary<string, int> Eigenschaften = new Dictionary<string, int>();   //char porperties
        public List<Talent> Talente = new List<Talent>();       //ist of talent objects (talents and spells)
        public List<Kampf> Kampftalente = new List<Kampf>();    //list of combat objects
        public List<Vorteil> Vorteile = new List<Vorteil>();

        public Dictionary<string, string> Proptable = new Dictionary<string, string>(); //KK -> Körperkraft


        public Character(String path)
        {
            Load(path); //load
            Proptable.Add("MU", "Mut");             //routing
            Proptable.Add("KL", "Klugheit");
            Proptable.Add("IN", "Intuition");
            Proptable.Add("CH", "Charisma");
            Proptable.Add("FF", "Fingerfertigkeit");
            Proptable.Add("GE", "Gewandtheit");
            Proptable.Add("KO", "Konstitution");
            Proptable.Add("KK", "Körperkraft");
        }

        public Character(Character c, string name, int stDv = 2)
        {
            Proptable.Add("MU", "Mut");             //routing
            Proptable.Add("KL", "Klugheit");
            Proptable.Add("IN", "Intuition");
            Proptable.Add("CH", "Charisma");
            Proptable.Add("FF", "Fingerfertigkeit");
            Proptable.Add("GE", "Gewandtheit");
            Proptable.Add("KO", "Konstitution");
            Proptable.Add("KK", "Körperkraft");
            this.Proptable.Add("**", "Klugheit");

            this.Name = name;
            foreach (var i in c.Eigenschaften)
            {
                this.Eigenschaften.Add(i.Key, i.Value + (int)Math.Round(Misc.Random(stDv)));
            }

            foreach (var i in c.Vorteile)
            {
                this.Vorteile.Add(new Vorteil(i.name, i.value + (int)Math.Round(Misc.Random(stDv))));
            }

            foreach (var i in c.Talente)
            {
                this.Talente.Add(new Talent(i.name, i.probe, i.value + (int)Math.Round(Misc.Random(stDv))));
            }

            foreach (var i in c.Kampftalente)
            {
                this.Kampftalente.Add(new Kampf(i.name, i.at + (int)Math.Round(Misc.Random(stDv)), i.pa + (int)Math.Round(Misc.Random(stDv))));
            }
        }

        private void Load(string path)
        {
            XmlTextReader reader = new XmlTextReader(path);
            while (reader.Read())   //read until he hits keywords
            {
                if (reader.NodeType == XmlNodeType.Element)
                {
                    
                    switch (reader.Name)
                    {
                        case "Wesen":
                            reader.Skip();
                            break;
                        case "held":
                            Name = reader.GetAttribute("name"); //name
                            break;
                        case "eigenschaft":
                            Eigenschaften.Add(
                                reader.GetAttribute("name"),
                                Convert.ToInt32(reader.GetAttribute("value"))
                                + Convert.ToInt32(reader.GetAttribute("mod")));
                            break;
                        case "vt":
                            reader.Read();
                            while (reader.Name.Equals("vorteil"))
                            {
                                try
                                {
                                    this.Vorteile.Add(new Vorteil(
                                        reader.GetAttribute("name"),
                                        Convert.ToInt32(reader.GetAttribute("value"))));
                                }
                                catch
                                {
                                    this.Vorteile.Add(new Vorteil(reader.GetAttribute("name")));
                                }
                                reader.Read();
                            }

                            break;
                        case "talentliste":
                            reader.Read();
                            while (reader.Name.Equals("talent"))
                            {
                                Talente.Add(
                                    new Talent(
                                        reader.GetAttribute("name"),
                                        reader.GetAttribute("probe").Remove(0, 2).Trim(')'),
                                        Convert.ToInt32(reader.GetAttribute("value"))));
                                reader.Read();
                            }
                            break;
                        case "zauberliste":
                            reader.Read();
                            while (reader.Name.Equals("zauber"))
                            {
                                Talente.Add(
                                    new Talent(
                                        reader.GetAttribute("name"),
                                        reader.GetAttribute("probe").Remove(0, 2).Trim(')'),
                                        Convert.ToInt32(reader.GetAttribute("value"))));
                                reader.Read();
                            }
                            break;
                        case "kampfwerte":
                            string atname = reader.GetAttribute("name");
                            reader.Read();
                            int at = Convert.ToInt32(reader.GetAttribute("value"));
                            reader.Read();
                            int pa = Convert.ToInt32(reader.GetAttribute("value"));
                            Kampftalente.Add(new Kampf(atname, at, pa));
                            break;
                    }
                }
            }
            

        }

        public string TestTalent(string talent, int erschwernis = 0)     //Talentprobe
        {
            try
            {
                var output = new StringBuilder();
                var sc = new SpellCorrect();
                var ttalent = Talente.OrderBy(x => sc.Compare(talent, x.name)).First();

                var deug = Talente.OrderBy(x => sc.Compare(talent, x.name));
                var fit = deug.Select(x => sc.Compare(talent, x.name));

                if (sc.Compare(talent, ttalent.name) > 94100) throw new Exception();

                var props = ttalent.Test(); //get the required properties
                int tap = ttalent.value; //get tap
                var werte = props.Select(p => this.Eigenschaften[this.Proptable[p]]).ToList();

                output.AppendFormat(
                    "{0} würfelt: {1} \n{2} - {3}   taw:{4} {5} \n",
                    this.Name,
                    ttalent.name,
                    ttalent.probe,
                    String.Join("/", werte),
                    ttalent.value,
                    erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

                output.Append("         ");
                tap -= erschwernis;
                int gesamt_erschwernis = tap;
                if (gesamt_erschwernis < 0)
                {
                    tap = 0;
                    for (int i = 0; i <= 2; i++) //foreach property, dice and tap 
                    {
                        int temp = dice.Roll();
                        int eigenschaft = Eigenschaften[Proptable[props[i]]];

                        if (eigenschaft - gesamt_erschwernis < temp)
                        {
                            tap -= temp - eigenschaft + gesamt_erschwernis;
                        }

                        output.Append($"[{temp}]"); //add to string
                    }

                    if (tap >= 0)
                    {
                        tap = 1;
                    }
                }
                else
                {
                    for (int i = 0; i <= 2; i++) //foreach property, dice and tap 
                    {
                        int temp = dice.Roll();
                        int eigenschaft = Eigenschaften[Proptable[props[i]]];

                        if (eigenschaft < temp)
                        {
                            tap -= temp - eigenschaft;
                        }
                        output.Append($"[{temp}]"); //add to string
                    }
                }

                tap = tap == 0 ? 1 : tap;

                output.AppendFormat(" tap: {0,2}", tap);

                return output.ToString(); //return output
            }
            catch (Exception)
            {
                throw new Exception(
                    $"{talent} nicht vorhanden! Besitzt {Name} {talent} nicht? \n Oder ist {talent} falsch geschrieben?");
            }
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var prop = this.Proptable[eigenschaft.ToUpper()];
            int tap = this.Eigenschaften[prop];
            output.AppendFormat(
                "{0}-Eigenschaftsprobe ew:{1} {2} \n", 
                prop, 
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            int roll = dice.Roll();
            output.Append($"Gewürfelt: {roll} übrig: {tap - roll - erschwernis}");
            return output.ToString();
        }

        public string Angriff(string talent, int erschwernis = 0)    //prety self explanetory
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            var attack = Kampftalente.OrderBy(x => sc.Compare(talent, x.name)).First();
            if (sc.Compare(talent, attack.name) > 94)
            {
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.at;
            output.AppendFormat("{0}-Angriff taw:{1} {2} \n", 
                attack.name, 
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            int temp = dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }
        public string Parade(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            var attack = Kampftalente.OrderBy(x => sc.Compare(talent, x.name)).First();

            if (sc.Compare(talent, attack.name) > 94)
            {
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.pa;
            output.AppendFormat(
                "{0}-Parade taw:{1} {2}\n", 
                attack.name, 
                tap, 
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            int temp = dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }

        public string Fernkampf(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            int fk = Eigenschaften["fk"];
            var attack = Talente.OrderBy(x => sc.Compare(talent, x.name)).First();
            if(sc.Compare(talent, attack.name) > 94)
            {
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.value;
            output.AppendFormat(
                "{0} taw:{1} {2} \n", 
                attack.name, 
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            tap -= erschwernis;
            int temp = dice.Roll();
            tap -= temp > fk ? temp - fk : 0;
            output.Append($"W20: {temp} tap: {tap}");
            return output.ToString();
        }

    }

}
