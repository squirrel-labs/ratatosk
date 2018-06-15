using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.Auxiliary
{
    using DiscoBot.Audio;
    using DiscoBot.Commands;
    using DiscoBot.DSA_Game;
    using DiscoBot.DSA_Game.Characters;

    public static class TalentEnumerableExtension
    {
        public static string ProbenTest(this IEnumerable<Talent> List, Character c, string talent, int erschwernis = 0)
        {
            var output = new StringBuilder();
            var sc = new SpellCorrect();
            var tTalent = List.OrderBy(x => sc.Compare(talent, x.Name)).First();

            if (sc.Compare(talent, tTalent.Name) > SpellCorrect.ErrorThreshold)
            {
                try
                {
                    SoundEffects.Play("Stupid");
                }
                catch { }
                return $"{c.Name} kann nicht {talent}...";
            }

            var props = tTalent.GetEigenschaften(); // get the required properties
            int tap = tTalent.Value; // get taw
            var werte = props.Select(p => c.Eigenschaften[c.PropTable[p]]).ToList();

            output.AppendFormat(
                "{0} würfelt: {1} \n{2} - {3}   taw:{4} {5} \n",
                c.Name,
                tTalent.Name,
                tTalent.Probe,
                string.Join("/", werte),
                tTalent.Value,
                erschwernis.Equals(0) ? string.Empty : "Erschwernis: " + erschwernis);

            output.Append("         ");
            tap -= erschwernis;
            int gesamtErschwernis = tap;
            if (gesamtErschwernis < 0)
            {
                tap = 0;
                for (int i = 0; i <= 2; i++)
                {
                    // foreach property, dice and tap 
                    int temp = Dice.Roll();
                    int eigenschaft = c.Eigenschaften[c.PropTable[props[i]]];

                    if (eigenschaft + gesamtErschwernis < temp)
                    {
                        tap -= temp - (eigenschaft + gesamtErschwernis);
                    }

                    output.Append($"[{temp}]"); // add to string
                }

                if (tap >= 0)
                {
                    tap = 1;
                }
            }
            else
            {
                for (int i = 0; i <= 2; i++)
                {
                    // foreach property, dice and tap 
                    int temp = Dice.Roll();
                    int eigenschaft = c.Eigenschaften[c.PropTable[props[i]]];

                    if (eigenschaft < temp)
                    {
                        tap -= temp - eigenschaft;
                    }

                    output.Append($"[{temp}]"); // add to string
                }
            }

            tap = (tap == 0) ? 1 : tap;

            if (tap < 0)
            {
                try
                {
                    SoundEffects.Play("Wrong");
                }
                catch { }
            }

            output.AppendFormat(" tap: {0,2}", tap);

            return output.ToString(); // return output
        }
    }
}
