namespace DiscoBot.Commands
{
    using System;
    using System.Linq;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;
    using DiscoBot.Characters;

    using Discord.Commands;

    public class NpcCommands : ModuleBase
    {
        [Command("npc"), Summary("Erstellt ein NPC")]
        [Alias("Npc", "NPc", "NPC", "nPC")]
        public Task RandomAsync([Summary("Create Random")] string npcName, int mean = 9, int stDv = 1)
        {
            Dsa.Chars.Add(new Npc(npcName, mean, stDv));
            return this.ReplyAsync($"{npcName} wurde zufällig generiert");
        }

        [Command("npc"), Summary("Erstellt ein NPC")]
        [Alias("Npc", "NPc", "NPC", "nPC")]
        public Task CopyAsync([Summary("Create Copy")] string npcName, string source, int stDv = 1)
        {
            if (Dsa.Chars.Exists(x => x.Name.Equals(npcName)))
            {
                throw new Exception("Char gibt es schon");
            }

            var comp = new SpellCorrect();
            var chr = Dsa.Chars.OrderBy(x => comp.Compare(x.Name, source)).First();
            Dsa.Chars.Add(new Character(chr as Character, npcName, stDv));
            return this.ReplyAsync($"{npcName} wurde als variierte Kopie von {source} erstellt");
        }
    }
}
