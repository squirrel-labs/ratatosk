namespace DiscoBot.Commands
{
    using System;
    using System.Collections.Generic;
    using System.Linq;
    using System.Text;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;
    using DiscoBot.Characters;

    using Discord.Commands;

    public class List : ModuleBase
    {
        [Command("list"), Summary("gibt eine Auflistung  aus")]
        public async Task ListAsync([Summary("Aktion")] string prop)
        {
            var res = new List<string>();

            var character = ((Character)Dsa.Chars.Find(x => x.Name.Equals(Dsa.Relation[this.Context.User.Username])));
            switch (prop.ToLower())
            {
                case "chars":
                case "Chars":
                    res.AddRange(Dsa.Chars.Select(x => x.Name));
                    break;
                case "e":
                case "eig":
                case "eigenschaft":
                case "eigenschaften":
                case "stat":
                case "stats":
                    res.Add(character.Name + ":");
                    res.AddRange(
                    //character.Eigenschaften.Select(s => s.Key + ":\t " + s.Value));
                    character.Eigenschaften.Take(9).Select(s => s.Key + ":\t " + s.Value));
                    res.Add("LE:\t " + character.Lebenspunkte);
                    break;
                case "t":
                case "ta":
                case "talent":
                case "zauber":
                    res.Add(character.Name + ":");
                    res.AddRange(
                        character.Talente.Select(s => s.Name + "\t " + s.Value + "\t " + s.Probe));
                    break;
                case "w":
                case "waffe":
                case "waffen":
                    res.Add(character.Name + ":");
                    res.AddRange(
                        character.Kampftalente.Select(s => s.Name));
                    break;
                case "fern":
                    res.Add(character.Name + ":");
                    res.AddRange(
                        character.Talente.Select(s => s.Name));
                    break;
                case "v":
                case "vt":
                case "vor":
                case "vorteil":
                    res.Add(character.Name + ":");
                    res.AddRange(
                        character.Vorteile
                        .Select(s => s.Name + "\t " + s.Value));// (s.Value == 0 ? string.Empty : s.Value.ToString())));
                    break;

                default:
                    res.Add($"Kommando {prop} nicht gefunden");
                    break;
            }
            
            var sb = new StringBuilder();
            foreach (string re in res)
            {
                if (sb.Length + re.Length > 1798)
                {
                    await this.ReplyTimedAsync(sb.ToString(), TimeSpan.FromSeconds(90));
                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            await this.ReplyTimedAsync(sb.ToString(), TimeSpan.FromSeconds(90));
        }
    }
}
