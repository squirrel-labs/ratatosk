namespace DiscoBot.DSA_Game.Characters
{
    using System;
    using System.Collections.Generic;
    using System.Linq;
    using System.Text;
    using System.Xml;

    using DiscoBot.Audio;
    using DiscoBot.Auxiliary;

    public class Character : ICharacter
    {
        public Character()
        {
            this.PropTable.Add("MU", "Mut");             // routing
            this.PropTable.Add("KL", "Klugheit");
            this.PropTable.Add("IN", "Intuition");
            this.PropTable.Add("CH", "Charisma");
            this.PropTable.Add("FF", "Fingerfertigkeit");
            this.PropTable.Add("GE", "Gewandtheit");
            this.PropTable.Add("KO", "Konstitution");
            this.PropTable.Add("KK", "Körperkraft");

        }

        public Character(string path) : this()
        {
            this.Load(path); // load
            this.Post_process(); // calculate derived values
        }

        public Character(Character c, string name, int stDv = 2) : this()
        {
            this.Name = name;
            foreach (var i in c.Eigenschaften)
            {
                this.Eigenschaften.Add(i.Key, i.Value + (int)Math.Round(RandomMisc.Random(stDv)));
            }

            foreach (var i in c.Vorteile)
            {
                this.Vorteile.Add(new Vorteil(i.Name, i.Value + (int)Math.Round(RandomMisc.Random(stDv))));
            }

            foreach (var i in c.Talente)
            {
                this.Talente.Add(new Talent(i.Name, i.Probe, i.Value + (int)Math.Round(RandomMisc.Random(stDv))));
            }

            foreach (var i in c.Zauber)
            {
                this.Zauber.Add(new Zauber(i.Name, i.Probe, i.Value + (int)Math.Round(RandomMisc.Random(stDv)), i.Complexity, i.Representation));
            }

            foreach (var i in c.Kampftalente)
            {
                this.Kampftalente.Add(new KampfTalent(i.Name, i.At + (int)Math.Round(RandomMisc.Random(stDv)), i.Pa + (int)Math.Round(RandomMisc.Random(stDv))));
            }

            this.Post_process(); // calculate derived values
        }

        public string Name { get; set; } // char name

        public int Lebenspunkte { get; set; }

        public int Astralpunkte { get; set; }

        public Dictionary<string, int> Eigenschaften { get; set; } = new Dictionary<string, int>();   // char properties

        public List<Talent> Talente { get; set; } = new List<Talent>();       // list of talent objects (talents)

        public List<Zauber> Zauber { get; set; } = new List<Zauber>();       // list of spell objects 

        public List<KampfTalent> Kampftalente { get; set; } = new List<KampfTalent>();    // list of combat objects

        public List<Vorteil> Vorteile { get; set; } = new List<Vorteil>();

        public Dictionary<string, string> PropTable { get; set; } = new Dictionary<string, string>(); // -> Körperkraft
        
        public string TestTalent(string talent, int erschwernis = 0)     // Talentprobe
        {
            return this.Talente.ProbenTest(this, talent, erschwernis);
        }

        public string TestZauber(string zauber, int erschwernis = 0)     // Talentprobe
        {
            return this.Zauber.ProbenTest(this, zauber, erschwernis);
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var prop = this.PropTable[eigenschaft.ToUpper()];
            int tap = this.Eigenschaften[prop];
            output.AppendFormat(
                "{0}-Eigenschaftsprobe ew:{1} {2} \n", 
                prop, 
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            int roll = Dice.Roll();
            output.Append($"Gewürfelt: {roll} übrig: {tap - roll - erschwernis}");
            return output.ToString();
        }

        public string Angriff(string talent, int erschwernis = 0)    // pretty self explanatory
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            var attack = this.Kampftalente.OrderBy(x => sc.Compare(talent, x.Name)).First();
            if (sc.Compare(talent, attack.Name) > SpellCorrect.ErrorThreshold)
            {
                SoundEffects.Play(Sound.Wrong).Wait();
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.At;
            output.AppendFormat(
                "{0}-Angriff taw:{1} {2} \n",
                attack.Name,
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            int temp = Dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }

        public string Parade(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            var attack = this.Kampftalente.OrderBy(x => sc.Compare(talent, x.Name)).First();

            if (sc.Compare(talent, attack.Name) > SpellCorrect.ErrorThreshold)
            {
                SoundEffects.Play(Sound.Wrong).Wait();
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.Pa;
            output.AppendFormat(
                "{0}-Parade taw:{1} {2}\n", 
                attack.Name, 
                tap, 
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            int temp = Dice.Roll();
            output.Append(temp - erschwernis);
            return output.ToString();
        }

        public string Fernkampf(string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            int fk = this.Eigenschaften["fk"];
            var attack = this.Talente.OrderBy(x => sc.Compare(talent, x.Name)).First();
            if (sc.Compare(talent, attack.Name) > SpellCorrect.ErrorThreshold)
            {
                SoundEffects.Play(Sound.Wrong).Wait();
                return $"{this.Name} kann nicht mit der Waffenart {talent} umgehen...";
            }

            int tap = attack.Value;
            output.AppendFormat(
                "{0} taw:{1} {2} \n", 
                attack.Name, 
                tap,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);
            tap -= erschwernis;
            int temp = Dice.Roll();
            tap -= temp > fk ? temp - fk : 0;
            output.Append($"W20: {temp} tap: {tap}");
            return output.ToString();
        }

        private void Post_process()
        {
            var LE_Wert = this.Eigenschaften["Lebensenergie"];
            var AE_Wert = this.Eigenschaften.First(s => s.Key.Contains("Astralenergie")).Value;

            //var KL_Wert = this.Eigenschaften.First(s => s.Key.Contains("Klugheit")).Value;
            var MU_Wert = this.Eigenschaften.First(s => s.Key.Contains("Mut")).Value;
            var IN_Wert = this.Eigenschaften.First(s => s.Key.Contains("Intuition")).Value;
            var CH_Wert = this.Eigenschaften.First(s => s.Key.Contains("Charisma")).Value;
            var KK_Wert = this.Eigenschaften["Körperkraft"];
            var KO__Wert = this.Eigenschaften["Konstitution"];

            this.Astralpunkte = 0;
            this.Lebenspunkte = LE_Wert + (int)(KO__Wert + (KK_Wert/2.0) + 0.5);

            if (this.Vorteile.Exists(x => x.Name.ToLower().Contains("zauberer")))
            {
                this.Astralpunkte = AE_Wert + (int)((MU_Wert + IN_Wert + CH_Wert) / 2.0 + 0.5);
            }

        }

            private void Load(string path)
        {
            var reader = new XmlTextReader(path);
            while (reader.Read())
            {
                // read until he hits keywords
                if (reader.NodeType != XmlNodeType.Element)
                {
                    continue;
                }

                switch (reader.Name)
                {
                    case "Wesen":
                        reader.Skip();
                        break;
                    case "held":
                        this.Name = reader.GetAttribute("name"); // name
                        break;
                    case "eigenschaft":
                        this.Eigenschaften.Add(
                            reader.GetAttribute("name") ?? throw new InvalidOperationException(),
                            Convert.ToInt32(reader.GetAttribute("value")) + Convert.ToInt32(reader.GetAttribute("mod")));
                        break;
                    case "vt":
                        reader.Read();
                        while (reader.Name.Equals("vorteil"))
                        {
                            try
                            {
                                this.Vorteile.Add(new Vorteil(
                                    reader.GetAttribute("name"),
                                  //  Convert.ToInt32(reader.GetAttribute("value"))));
                                reader.GetAttribute("value")));
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
                            this.Talente.Add(
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
                            this.Zauber.Add(
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
                        string atName = reader.GetAttribute("name");
                        reader.Read();
                        int at = Convert.ToInt32(reader.GetAttribute("value"));
                        reader.Read();
                        int pa = Convert.ToInt32(reader.GetAttribute("value"));
                        this.Kampftalente.Add(new KampfTalent(atName, at, pa));
                        break;
                }
            }
        }
    }
}
