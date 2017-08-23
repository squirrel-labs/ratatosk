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

        public string name; //charname
        public Dictionary<string, int> eigenschaften = new Dictionary<string, int>();   //char porperties
        public List<Talent> talente = new List<Talent>();       //ist of talent objects (talents and spells)
        public List<Kampf> kampftalente = new List<Kampf>();    //list of combat objects

        public Dictionary<string, string> Proptable = new Dictionary<string, string>(); //KK -> Körperkraft


        public Char(String path )
        {

            Load(path); //load
        }

        private void Load(string path)
        {
            XmlTextReader reader = new XmlTextReader(path);
            while (reader.Read())   //read until he hits keywords
            {
                if (reader.NodeType == XmlNodeType.Element)
                    switch (reader.Name)
                    {
                        case "held":
                            name = reader.GetAttribute("name"); //name
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
                        case "zauberliste":
                            reader.Read();
                            while (reader.Name.Equals("zauber"))
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
            Proptable.Add("MU", "Mut");             //routing
            Proptable.Add("KL", "Klugheit");
            Proptable.Add("IN", "Intuition");
            Proptable.Add("CH", "Charisma");
            Proptable.Add("FF", "Fingerfertigkeit");
            Proptable.Add("GE", "Gewandtheit");
            Proptable.Add("KO", "Konstitution");
            Proptable.Add("KK", "Körperkraft");

        }
        public string TestTalent(string talent)     //Talentprobe
        {
            var output = new StringBuilder();
            var ttalentlist = talente.Select(v => v.CheckName(talent)).ToList(); //find the talent
            int error = ttalentlist.Min();
            var ttalent = talente[ttalentlist.IndexOf(error)];
            var props = ttalent.Test();                             //get the required properties
            int tap = ttalent.value;        //get tap
            output.AppendFormat("{0} {1} taw:{2} error: \n", ttalent.name,ttalent.probe,ttalent.value,error);
            for (int i = 0; i <= 2; i++)    //foreach property, dice and tap 
            {
                int temp = dice.Roll();
                int eigenschaft = eigenschaften[Proptable[props[i]]];
                if (eigenschaft < temp)
                    tap -= temp - eigenschaft;
                output.Append(temp + " ");      //add to string
            }
            output.AppendFormat("tap: {0}",tap);
            if (error == 100)
                return talent + " nicht gefunden!";
            return output.ToString();       //return output
        }
        public string Angriff(string talent)    //prety self explanetory
        {
            var output = new StringBuilder();
            var attack = kampftalente.Find(x => x.name.ToLower().Equals(talent.ToLower()));
            int tap = attack.at;
            output.AppendFormat("{0}-Angriff taw:{1}  \n", attack.name, tap);
            int temp = dice.Roll();
            output.Append(temp );
            return output.ToString();
        }
        public string Parade(string talent)
        {
            var output = new StringBuilder();
            var attack = kampftalente.Find(x => x.name.ToLower().Equals(talent.ToLower()));
            int tap = attack.pa;
            output.AppendFormat("{0}-Parade taw:{1} \n", attack.name, tap);
            int temp = dice.Roll();
            output.Append(temp);
            return output.ToString();
        }
        public string Fernkampf(string talent,int erschwernis=0)
        {
            var output = new StringBuilder();
            int fk = eigenschaften["fk"];
            var attack = talente.Find(v => v.name.ToLower().Equals(talent.ToLower()));
            int tap = attack.value ;
            output.AppendFormat("{0} taw:{1} erschwernis:{2} \n", attack.name, tap,erschwernis);
            tap -= erschwernis;
            int temp = dice.Roll();
            tap -= temp>fk?temp-fk:0;
            output.Append(temp + " tap:" + tap);
            return output.ToString();
        }

    }
    
}
