using System.Collections.Generic;
using System.Linq;
using System.Text;
using DSALib.Auxiliary;
using DSALib.DSA_Game;
using DSALib.DSA_Game.Characters;

namespace DSALib.Commands {
    public class HeldList {
        public static string ListAsync(ulong id, params string[] prop_list) {
            var res = new List<string>();

            var character = Dsa.GetCharacter(id) as Character;

            var first_column_width = 18;


            if (prop_list.Length == 0 || prop_list[0].ToLower().StartsWith("all") ||
                prop_list[0].ToLower().StartsWith("brief") || prop_list[0].ToLower().StartsWith("zettel")) {
                res.Add(character.Name + ":\n");
                //Eigenschaften
                res.AddRange(
                    character.Eigenschaften.Take(9).Select(s => s.Key + ":\t " + s.Value));
                res.Add("");
                //LE/AE
                res.Add("LE:\t " + character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis);
                if (character.Astralpunkte_Basis > 0)
                    res.Add("AE:\t " + character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis);
                res.Add("");
                //Kampfwerte
                res.Add("".AddSpaces(first_column_width) + " AT/PA");
                res.AddRange(
                    character.Kampftalente.Select(s =>
                        s.Name.AddSpaces(first_column_width) + " " + s.At.ToString().AddSpacesAtHead(2) + "/" +
                        s.Pa.ToString().AddSpacesAtHead(2)));
                res.Add("");
                //Fernkampf
                res.Add("".AddSpaces(first_column_width) + " FK");
                res.AddRange(
                    character.Talente.Where(x => x.IstFernkampftalent()).Select(s =>
                        s.Name.AddSpaces(first_column_width) + " " +
                        (character.Eigenschaften["fk"] + s.Value).ToString().AddSpacesAtHead(2)));
                res.Add("");
                //Vorteile
                res.AddRange(
                    character.Vorteile
                        .Select(s => s.Name + "\t " + s.Value));
                res.Add("");
                //Talente
                res.AddRange(
                    character.Talente.Select(s =>
                        (s.Name.AddSpaces(first_column_width) + " " + s.Value).AddSpaces(first_column_width + 5) + " " +
                        s.Probe));
                res.Add("");
                //evtl Zauber
                if (character.Zauber.Count > 0)
                    res.AddRange(
                        character.Zauber.Select(s =>
                            (s.Name.AddSpaces(first_column_width) + " " + s.Value).AddSpaces(first_column_width + 5) +
                            " " + s.Probe));
            }
            else if (prop_list[0].ToLower().StartsWith("man") || prop_list[0].ToLower().StartsWith("help") ||
                     prop_list[0].ToLower().StartsWith("hilf")) {
                return "```xl\n" + Help.Get_Specific_Help("Held") + "\n```";
            }
            else {
                res.Add(character.Name + ":\n");

                foreach (var prop in prop_list) {
                    switch (prop.ToLower()) {
                        case "e":
                        case "eig":
                        case "eigenschaft":
                        case "eigenschaften":
                            res.AddRange(
                                character.Eigenschaften.Take(8).Select(s => s.Key + ":\t " + s.Value));
                            break;
                        case "stat":
                        case "stats":
                            res.AddRange(
                                character.Eigenschaften.Take(9).Select(s => s.Key + ":\t " + s.Value));
                            res.Add("");
                            res.Add("LE:\t " + character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis);
                            if (character.Astralpunkte_Basis > 0)
                                res.Add("AE:\t " + character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis);
                            break;
                        case "le":
                            res.Add("LE:\t " + character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis);
                            break;
                        case "ae":
                            res.Add("AE:\t " + character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis);
                            break;
                        case "t":
                        case "ta":
                        case "talent":
                        case "talente":
                            res.AddRange(
                                character.Talente.Select(s =>
                                    (s.Name.AddSpaces(first_column_width) + " " + s.Value).AddSpaces(
                                        first_column_width + 5) + " " + s.Probe));
                            break;
                        case "zauber":
                        case "z":
                            res.AddRange(
                                character.Zauber.Select(s =>
                                    (s.Name.AddSpaces(first_column_width) + " " + s.Value).AddSpaces(
                                        first_column_width + 5) + " " + s.Probe));
                            break;
                        case "w":
                        case "waffe":
                        case "waffen":
                        case "kampf":
                        case "kampfwert":
                        case "kampfwerte":
                            res.Add("".AddSpaces(first_column_width) + " AT/PA");
                            res.AddRange(
                                character.Kampftalente.Select(s =>
                                    s.Name.AddSpaces(first_column_width) + " " + s.At.ToString().AddSpacesAtHead(2) +
                                    "/" + s.Pa.ToString().AddSpacesAtHead(2)));
                            break;
                        case "f":
                        case "fern":
                            res.Add("".AddSpaces(first_column_width) + " FK");
                            res.AddRange(
                                character.Talente.Where(x => x.IstFernkampftalent()).Select(s =>
                                    s.Name.AddSpaces(first_column_width) + " " +
                                    (character.Eigenschaften["fk"] + s.Value).ToString().AddSpacesAtHead(2)));
                            break;
                        case "v":
                        case "vt":
                        case "vor":
                        case "vorteil":
                        case "vorteile":
                        case "nachteil":
                        case "nachteile":
                            res.AddRange(
                                character.Vorteile
                                    .Select(s => s.Name + "\t " + s.Value));
                            break;

                        default:
                            res.Add($"Kommando {prop} nicht gefunden");
                            break;
                    }

                    res.Add("");
                }
            }


            var sb = new StringBuilder();
            foreach (var re in res) sb.AppendLine(re);

            return sb.ToString();
            /* 
            if (persist == 1)
            {
                await this.ReplyAsync(res, true);
            }
            else
            {
                await this.ReplyAsync(res, TimeSpan.FromSeconds(90));
            }*/
        }
    }
}