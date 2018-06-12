namespace DiscoBot.Commands
{
    using System.Threading.Tasks;

    using DiscoBot.DSA_Game;

    using Discord.Commands;

    public class ProbenTest : ModuleBase
    {
        [Command("t"), Summary("Würfelt ein Talent-/Zauberprobe")]
        [Alias("T", "Talent", "talent", "versuche")]
        public Task TalentAsync([Summary("Talent oder Zaubername")] string talent, int erschwernis = 0)
        {
            string res;
            try
            {
                res = Gm.CheckCommand(
                    Dsa.Relation[this.Context.User.Username],
                    CommandTypes.Talent,
                    talent,
                    erschwernis);
            }
            catch
            {
                res = Gm.CheckCommand(
                    Dsa.Relation["Tardis"],
                    CommandTypes.Talent,
                    talent,
                    erschwernis);
            }

            return this.ReplyAsync("```xl\n" + res + "\n```");
        }

        [Command("Zauber"), Summary("Würfelt ein Zauberprobe")]
        [Alias("Z", "zauber", "z")]
        public Task ZauberAsync([Summary("Zaubername")] string zauber, int erschwernis = 0)
        {
            string res;
            try
            {
                res = Gm.CheckCommand(
                    Dsa.Relation[this.Context.User.Username],
                    CommandTypes.Zauber,
                    zauber,
                    erschwernis);
            }
            catch
            {
                res = Gm.CheckCommand(
                    Dsa.Relation["Tardis"],
                    CommandTypes.Zauber,
                    zauber,
                    erschwernis);
            }

            return this.ReplyAsync("```xl\n" + res + "\n```");
        }

        [Command("e"), Summary("Würfelt eine Eigenschaftsprobe")]
        [Alias("E", "Eigenschaft", "eigenschaft", "eigen")]
        public Task EigenschaftAsync([Summary("Eigenschaftskürzel und Erschwernis")] string talent, int erschwernis = 0)
        {
            var chr = Dsa.Chars.Find(x => x.Name.Equals(Dsa.Relation[this.Context.User.Username]));
            string res = chr.TestEigenschaft(talent, erschwernis);
            return this.ReplyAsync("```xl\n" + res + "\n```");
        }

        [Command("a"), Summary("Würfelt ein Angriff")]
        [Alias("A", "At", "at", "Angriff", "angriff", "attackiere_mit", "attacke", "Attacke")]
        public Task AngriffAsync([Summary("Weapon")] string weapon, int erschwernis = 0)
        {
            return this.ReplyAsync("```xl\n" + Dsa.Chars.Find(x => x.Name.Equals(Dsa.Relation[this.Context.User.Username])).Angriff(weapon, erschwernis) + "\n```");
        }

        [Command("p"), Summary("Würfelt eine Parade Probe")]
        [Alias("P", "Parade", "parade", "pariere_mit")]
        public Task ParadeAsync([Summary("Parade Weapon")] string talent, int erschwernis = 0)
        {
            return this.ReplyAsync("```xl\n" + Dsa.Chars.Find(x => x.Name.Equals(Dsa.Relation[this.Context.User.Username])).Parade(talent, erschwernis) + "\n```");
        }

        [Command("f"), Summary("Führt eine Fernkampfprobe aus")]
        [Alias("F", "fern", "Fern", "Schuss", "schuss", "fernkampf", "Fernkampf", "schieße", "schieße_mit")]
        public Task FernkampfAsync([Summary("Fernkampfwaffe")] string waffe, int erschwernis = 0)
        {
            return this.ReplyAsync("```xl\n" + Dsa.Chars.Find(x => x.Name.Equals(Dsa.Relation[this.Context.User.Username])).Fernkampf(waffe, erschwernis) + "\n```");
        }
    }
}
