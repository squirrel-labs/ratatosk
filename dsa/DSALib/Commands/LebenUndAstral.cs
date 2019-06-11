using System;
using DSALib.Auxiliary;
using DSALib.DSA_Game;
using DSALib.Characters;

namespace DSALib.Commands
{
    public class LE
    {
        public static string LEAsync(ulong id, string modifier)
        {
            //This is the string that will be printed
            var res = "";


            //Get the actual text
            res += Dsa.GetCharacter(id).get_LE_Text(modifier);


            return res;
        }
    }

    public class AE
    {
        public static string AEAsync(ulong id, string modifier)
        {
            //This is the string that will be printed
            var res = "";


            //Get the actual text
            res += Dsa.GetCharacter(id).get_AE_Text(modifier);

            return res;
        }
    }

    public static class StatExtension
    {
        public static string get_LE_Text(this ICharacter c, string prop)
        {
            var res = "";
            var comp = new SpellCorrect();
            var character = c;

            res += character.Name + ":\n";

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
                        character.Lebenspunkte_Aktuell = character.Lebenspunkte_Aktuell +
                                                         Convert.ToInt32(prop.Substring(1, prop.Length - 1));
                    }
                    else
                    {
                        var temp = character.Lebenspunkte_Aktuell + Convert.ToInt32(prop) -
                                   character.Lebenspunkte_Basis;
                        //Stop from overflow overflow
                        if (temp > 0 && prop.StartsWith("+"))
                        {
                            character.Lebenspunkte_Aktuell =
                                character.Lebenspunkte_Basis > character.Lebenspunkte_Aktuell
                                    ? character.Lebenspunkte_Basis
                                    : character.Lebenspunkte_Aktuell;
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
                res += "LE: " + character.Lebenspunkte_Aktuell + "/" + character.Lebenspunkte_Basis;
            }

            return res;
        }

        public static string get_AE_Text(this ICharacter c, string prop)
        {
            var res = "";
            var comp = new SpellCorrect();
            var character = c;

            res += character.Name + ":\n";

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
                        character.Astralpunkte_Aktuell = character.Astralpunkte_Aktuell +
                                                         Convert.ToInt32(prop.Substring(1, prop.Length - 1));
                    }
                    else
                    {
                        var temp = character.Astralpunkte_Aktuell + Convert.ToInt32(prop) -
                                   character.Astralpunkte_Basis;
                        //Stop from overflow overflow
                        if (temp > 0 && prop.StartsWith("+"))
                        {
                            character.Astralpunkte_Aktuell =
                                character.Astralpunkte_Basis > character.Astralpunkte_Aktuell
                                    ? character.Astralpunkte_Basis
                                    : character.Astralpunkte_Aktuell;
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
                res += "AE: " + character.Astralpunkte_Aktuell + "/" + character.Astralpunkte_Basis;
            }


            return res;
        }
    }
}