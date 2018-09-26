namespace DiscoBot.Commands
{
    using System;
    using System.Linq;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;
    using DiscoBot.DSA_Game;
    using DiscoBot.DSA_Game.Characters;

    using Discord.Commands;
    using DSALib.Characters;

    public class LE : ModuleBase
    {
        [Command("LE"), Summary("Ändert aktuellen Lebenspunktestand")]
        [Alias("le", "leben", "LP", "lp", "Le", "Lp")]

        public async Task LEAsync([Summary("LE Modifier")] string prop = "", string s = "")
        {
            //This is the string that will be printed
            string res = "";

            if (prop.ToLower().Equals("help") || prop.ToLower().Equals("man"))
            {
                await this.ReplyAsync("```xl\n" + Help.Get_Specific_Help("LE") + "\n```");
                return;
            }

            //In case the input is badly formated
            prop = prop.Trim() + s.Trim();


            //Get the actual text
            res += Dsa.Chars.OrderBy(x => SpellCorrect.CompareEasy(Dsa.Session.Relation[this.Context.User.Username], x.Name)).First().get_LE_Text(prop);


            await this.ReplyAsync("```xl\n" + res + "\n```");
        }
    }

    public class AE : ModuleBase
    {
        [Command("AE"), Summary("Ändert aktuellen Astralpunktestand")]
        [Alias("ae", "astral", "ASP", "Asp", "asp", "Astral")]

        public async Task AEAsync([Summary("AE Modifier")] string prop = "", string s = "")
        {
            //This is the string that will be printed
            string res = "";

            if (prop.ToLower().Equals("help") || prop.ToLower().Equals("man"))
            {
                await this.ReplyAsync("```xl\n" + Help.Get_Specific_Help("AE") + "\n```");
                return;
            }

            //Incase the input is badly formated
            prop = prop.Trim() + s.Trim();


            //Get the actual text
            res += Dsa.Chars.OrderBy(x => SpellCorrect.CompareEasy(Dsa.Session.Relation[this.Context.User.Username], x.Name)).First().get_AE_Text(prop);



            await this.ReplyAsync("```xl\n" + res + "\n```");
        }
    }

    public static class StatExtension
    {
        public static string get_LE_Text(this ICharacter c, string prop)
        {
            string res = "";
            var comp = new SpellCorrect();
            var character = c;

            res += (character.Name + ":\n");

            //If there is actual input we process it
            if (prop.Length > 0)
            {
                res += "LE: ";
                res += character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis + " -> ";

                // Apply a change to current value
                if (prop.StartsWith("+") || prop.StartsWith("-"))
                {
                    //Allow overflowing the max
                    if (prop.StartsWith("++"))
                    {
                        character.Lebenspunkte_Aktuell = character.Lebenspunkte_Aktuell + Convert.ToInt32(prop.Substring(1, prop.Length - 1));
                    }
                    else
                    {
                        int temp = character.Lebenspunkte_Aktuell + Convert.ToInt32(prop) - character.Lebenspunkte_Basis;
                        //Stop from overflow overflow
                        if (temp > 0 && prop.StartsWith("+"))
                        {
                            character.Lebenspunkte_Aktuell = (character.Lebenspunkte_Basis > character.Lebenspunkte_Aktuell) ? character.Lebenspunkte_Basis : character.Lebenspunkte_Aktuell;
                            res += " Maximale Lebenspunkte sind erreicht ";
                        }
                        //Simply apply change
                        else
                        {
                            character.Lebenspunkte_Aktuell = character.Lebenspunkte_Aktuell + Convert.ToInt32(prop);
                        }
                    }

                    res += character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis;
                }
                else
                {
                    // Set to new value regardless of original
                    character.Lebenspunkte_Aktuell = Convert.ToInt32(prop);

                    res += character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis;
                }
            }
            //If no value is passed, the curent value is displayed
            else
            {
                res += ("LE: " + character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis);
            }

            return res;
        }
        public static string get_AE_Text(this ICharacter c, string prop)
        {
            string res = "";
            var comp = new SpellCorrect();
            var character = c;

            res += (character.Name + ":\n");

            //If there is actual input we process it
            if (prop.Length > 0)
            {

                res += "AE: ";
                res += character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis + " -> ";

                // Apply a change to current value
                if (prop.StartsWith("+") || prop.StartsWith("-"))
                {
                    //Allow overflowing the max
                    if (prop.StartsWith("++"))
                    {
                        character.Astralpunkte_Aktuell = character.Astralpunkte_Aktuell + Convert.ToInt32(prop.Substring(1, prop.Length - 1));
                    }
                    else
                    {
                        int temp = character.Astralpunkte_Aktuell + Convert.ToInt32(prop) - character.Astralpunkte_Basis;
                        //Stop from overflow overflow
                        if (temp > 0 && prop.StartsWith("+"))
                        {
                            character.Astralpunkte_Aktuell = (character.Astralpunkte_Basis > character.Astralpunkte_Aktuell) ? character.Astralpunkte_Basis : character.Astralpunkte_Aktuell;
                            res += " Maximale Astralpunkte sind erreicht ";
                        }
                        //Simply apply change
                        else
                        {
                            character.Astralpunkte_Aktuell = character.Astralpunkte_Aktuell + Convert.ToInt32(prop);
                        }
                    }

                    if (character.Astralpunkte_Aktuell < 0)
                    {
                        res += "Nicht genügend Astralpunkte! ";
                        character.Astralpunkte_Aktuell = 0;
                    }

                    res += character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis;

                }
                //Set to new value regardless of original
                else
                {
                    character.Astralpunkte_Aktuell = Convert.ToInt32(prop);

                    res += character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis;
                }
            }
            //If no value is passed, the curent value is displayed
            else
            {
                res += ("AE: " + character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis);
            }


            return res;
        }
    }
}



