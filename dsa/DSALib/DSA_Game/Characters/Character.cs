using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Xml;
using DSACore.Auxiliary;
using DSALib.Auxiliary;
using DSALib.Characters;
using DSALib.Models.Dsa;

namespace DSALib.DSA_Game.Characters
{
    public class Character : Being, ICharacter
    {
        public Character()
        {
            PropTable.Add("MU", "Mut"); // routing
            PropTable.Add("KL", "Klugheit");
            PropTable.Add("IN", "Intuition");
            PropTable.Add("CH", "Charisma");
            PropTable.Add("FF", "Fingerfertigkeit");
            PropTable.Add("GE", "Gewandtheit");
            PropTable.Add("KO", "Konstitution");
            PropTable.Add("KK", "Körperkraft");
        }

        public Character(string path) : this()
        {
            Load(new MemoryStream(File.ReadAllBytes(path))); // load
            Post_process(); // calculate derived values
        }

        public Character(MemoryStream stream) : this()
        {
            Load(stream); // load
            Post_process(); // calculate derived values
        }

        public Character(Character c, string name, int stDv = 2) : this()
        {
            Name = name;
            foreach (var i in c.Eigenschaften)
                Eigenschaften.Add(i.Key, i.Value + (int) Math.Round(RandomMisc.Random(stDv)));

            foreach (var i in c.Vorteile)
                Vorteile.Add(new Vorteil(i.Name, i.Value + (int) Math.Round(RandomMisc.Random(stDv))));

            foreach (var i in c.Talente)
                Talente.Add(new Talent(i.Name, i.Probe, i.Value + (int) Math.Round(RandomMisc.Random(stDv))));

            foreach (var i in c.Zauber)
                Zauber.Add(new Zauber(i.Name, i.Probe, i.Value + (int) Math.Round(RandomMisc.Random(stDv)),
                    i.Complexity, i.Representation));

            foreach (var i in c.Kampftalente)
                Kampftalente.Add(new KampfTalent(i.Name, i.At + (int) Math.Round(RandomMisc.Random(stDv)),
                    i.Pa + (int) Math.Round(RandomMisc.Random(stDv))));

            Post_process(); // calculate derived values
        }

        public Dictionary<string, int> Eigenschaften { get; set; } = new Dictionary<string, int>(); // char properties

        public List<Talent> Talente { get; set; } = new List<Talent>(); // list of talent objects (talents)

        public List<Zauber> Zauber { get; set; } = new List<Zauber>(); // list of spell objects 

        public List<KampfTalent> Kampftalente { get; set; } = new List<KampfTalent>(); // list of combat objects

        public List<Vorteil> Vorteile { get; set; } = new List<Vorteil>();

        public Dictionary<string, string> PropTable { get; set; } = new Dictionary<string, string>(); // -> Körperkraft

        public string TestTalent(string talent, int erschwernis = 0) // Talentprobe
        {
            return Talente.ProbenTest(this, talent, erschwernis);
        }

        public string TestZauber(string zauber, int erschwernis = 0) // Talentprobe
        {
            return Zauber.ProbenTest(this, zauber, erschwernis);
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var prop = PropTable[eigenschaft.ToUpper()];
            var tap = Eigenschaften[prop];
            output.AppendFormat(
                "{0}-Eigenschaftsprobe ew:{1} {2} \n",
                prop,
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            var roll = Dice.Roll();
            output.Append($"Gewürfelt: {roll} übrig: {tap - roll - erschwernis}");
            return output.ToString();
        }

        public string Angriff(string talent, int erschwernis = 0) // pretty self explanatory
        {
            var output = new StringBuilder();
            if (!Kampftalente.TryMatch(out var iattack, talent))
                return $"{Name} kann nicht mit der Waffenart {talent} umgehen...";
            var attack = (KampfTalent) iattack;
            var tap = attack.At;
            output.AppendFormat(
                "{0}-Angriff taw:{1} {2} \n",
                attack.Name,
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            var temp = Dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }

        public string Parade(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();

            if (Kampftalente.TryMatch(out var iAttack , talent))
                return $"{Name} kann nicht mit der Waffenart {talent} umgehen...";


            var attack = (KampfTalent) iAttack;
            var tap = attack.Pa;
            output.AppendFormat(
                "{0}-Parade taw:{1} {2}\n",
                attack.Name,
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            var temp = Dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }

        public string Fernkampf(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var fk = Eigenschaften["fk"];
            if (! Talente.TryMatch(out var iAttack, talent))
                return $"{Name} kann nicht mit der Waffenart {talent} umgehen...";

            var attack = (Talent) iAttack;
            var tap = attack.Value;
            output.AppendFormat(
                "{0} taw:{1} {2} \n",
                attack.Name,
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            tap -= erschwernis;
            var temp = Dice.Roll();
            tap -= temp > fk ? temp - fk : 0;
            output.Append($"W20: {temp} tap: {tap}");
            return output.ToString();
        }

        private void Post_process()
        {
            var LE_Wert = Eigenschaften["Lebensenergie"];
            var AE_Wert = Eigenschaften.First(s => s.Key.Contains("Astralenergie")).Value;

            //var KL_Wert = this.Eigenschaften.First(s => s.Key.Contains("Klugheit")).Value;
            var MU_Wert = Eigenschaften.First(s => s.Key.Contains("Mut")).Value;
            var IN_Wert = Eigenschaften.First(s => s.Key.Contains("Intuition")).Value;
            var CH_Wert = Eigenschaften.First(s => s.Key.Contains("Charisma")).Value;
            var KK_Wert = Eigenschaften["Körperkraft"];
            var KO__Wert = Eigenschaften["Konstitution"];

            Astralpunkte_Basis = 0;

            Ausdauer_Basis = 0;

            Lebenspunkte_Basis = LE_Wert + (int) (KO__Wert + KK_Wert / 2.0 + 0.5);

            if (Vorteile.Exists(x => x.Name.ToLower().Contains("zauberer")))
                Astralpunkte_Basis = AE_Wert + (int) ((MU_Wert + IN_Wert + CH_Wert) / 2.0 + 0.5);

            Lebenspunkte_Aktuell = Lebenspunkte_Basis;
            Astralpunkte_Aktuell = Astralpunkte_Basis;
            Ausdauer_Aktuell = Ausdauer_Basis;
        }


        private void Load(MemoryStream stream)
        {
            var reader = new XmlTextReader(stream);
            while (reader.Read())
            {
                // read until he hits keywords
                if (reader.NodeType != XmlNodeType.Element) continue;

                switch (reader.Name)
                {
                    case "Wesen":
                        reader.Skip();
                        break;
                    case "held":
                        Name = reader.GetAttribute("name"); // name
                        break;
                    case "eigenschaft":
                        Eigenschaften.Add(
                            reader.GetAttribute("name") ?? throw new InvalidOperationException(),
                            Convert.ToInt32(reader.GetAttribute("value")) +
                            Convert.ToInt32(reader.GetAttribute("mod")));
                        break;
                    case "vt":
                        reader.Read();
                        while (reader.Name.Equals("vorteil"))
                        {
                            try
                            {
                                Vorteile.Add(new Vorteil(
                                    reader.GetAttribute("name"),
                                    //  Convert.ToInt32(reader.GetAttribute("value"))));
                                    reader.GetAttribute("value")));
                            }
                            catch
                            {
                                Vorteile.Add(new Vorteil(reader.GetAttribute("name")));
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
                                    reader.GetAttribute("probe")?.Remove(0, 2).Trim(')'),
                                    Convert.ToInt32(reader.GetAttribute("value"))));
                            reader.Read();
                        }

                        break;
                    case "zauberliste":
                        reader.Read();
                        while (reader.Name.Equals("zauber"))
                        {
                            Zauber.Add(
                                new Zauber(
                                    reader.GetAttribute("name"),
                                    reader.GetAttribute("probe")?.Remove(0, 2).Trim(')'),
                                    Convert.ToInt32(reader.GetAttribute("value")),
                                    reader.GetAttribute("k").ToCharArray()[0],
                                    reader.GetAttribute("repraesentation")));
                            reader.Read();
                        }

                        break;
                    case "kampfwerte":
                        var atName = reader.GetAttribute("name");
                        reader.Read();
                        var at = Convert.ToInt32(reader.GetAttribute("value"));
                        reader.Read();
                        var pa = Convert.ToInt32(reader.GetAttribute("value"));
                        Kampftalente.Add(new KampfTalent(atName, at, pa));
                        break;
                }
            }
        }
    }
}