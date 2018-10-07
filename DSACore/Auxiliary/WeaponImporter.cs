using DSACore.Models.Database;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using DSACore.FireBase;
using Group = System.Text.RegularExpressions.Group;

namespace DSACore.Auxiliary
{
    public class WeaponImporter
    {
        private List<MeleeWeapon> Weapons = new List<MeleeWeapon>();
        private List<RangedWeapon> Range = new List<RangedWeapon>();

        public async Task DownloadWeapons()
        {
            var client = new HttpClient();

            

            for (int i = 1; i <= 25; i++)
            {
                var responseString = await client.GetStringAsync("http://diarium.eu/dsa4-forge/ajax/categoryChanged/" + i);

                Regex talentRegex = new Regex(@"(?<=<option value="")([0-9]*)("">)(.*?)(?=<)");
                //Regex idsRegex = new Regex(@"(?<=<option value=\"")([0-9]*)");


                var talentMatch = talentRegex.Matches(responseString);
                //var idMatch = idsRegex.Matches(responseString);

                var lines = new List<string>();
                var ids = new List<int>();

                foreach (var matchGroup in talentMatch.ToList())
                {
                    if (matchGroup.Success)
                    {
                        lines.Add(matchGroup.Groups[3].Value);
                        ids.Add(int.Parse(matchGroup.Groups[1].Value));
                    }
                }



                for (int j = 0; j < lines.Count; j++)
                {
                    var talent = lines[j];

                    var values = await client.GetStringAsync($"http://diarium.eu/dsa4-forge/ajax/calculate/" + i + "/" + ids[j] + "/0/0/0/0/0/10/0/0/0");

                    values = Regex.Unescape(values.Replace(@"\t", ""));
                    // ... Use named group in regular expression.
                    Regex expression = new Regex(@"(((?<=(<td>))|(?<=(<td style=\""padding:2px\"">))).*?(?=<\/td>))|((?<=<span style=\""font-weight:bold;text-decoration:underline;\"">).*?(?=<\/span>))");

                    // ... See if we matched.
                    var matches = expression.Matches(values).Select(x => x.ToString()).ToList();

                    // ... Get group by name.
                    await AddMelee(i, talent, matches);
                    Console.Write(j + ",");
                    //await Task.Delay(TimeSpan.FromSeconds(5));

                }

                Console.WriteLine($"{i}: {ids.Count} => {Weapons.Count}");
                //await Task.Delay(TimeSpan.FromSeconds(5));
            }

            Console.ReadLine();
        }

        private async Task AddMelee(int i, string talent, List<string> matches)
        {
            string name = talent.Replace(' ', '_').Replace(".", "");
            if (!matches[1].Equals(string.Empty))
            {
                var temp = new MeleeWeapon(
                    name,
                    matches[1],
                    int.TryParse(matches[10], out int weight) ? weight : 0,
                    matches[0].Split(':', StringSplitOptions.RemoveEmptyEntries).First(),
                    matches[11])
                {
                    INI = int.TryParse(matches[3], out int ini) ? ini : 0,
                    MW = matches[4],
                    TpKK = matches[2]
                };

                Weapons.Add(temp);
                await Database.AddWeapon(temp);
            }
            /*if (i > 23)
            {
                var range = new RangedWeapon(
                    name,
                    matches[13],
                    int.TryParse(matches[10], out int weight) ? weight : 0,
                    matches[0].Split(':', StringSplitOptions.RemoveEmptyEntries).First(),
                    matches[11])
                {
                    AtMod = int.TryParse(matches[10], out int atMod) ? atMod : 0,
                    KKMod = int.TryParse(matches[11], out int kkMod) ? kkMod : 0,
                    AtReach = matches[3],
                    TpReach = matches[4],
                    LoadTime = int.TryParse(matches[5], out int loadTime) ? loadTime : 0
                };
                Range.Add(range);
                await Database.AddWeapon(range);
                return;
            }*/
            if (i > 18)
            {
                var range = new RangedWeapon(
                    name,
                    matches[13].Replace(' ', '+'),
                    int.TryParse(matches[10], out int weight) ? weight : 0,
                    matches[0].Split(':', StringSplitOptions.RemoveEmptyEntries).First(),
                    matches[11])
                {
                    AtMod = int.TryParse(matches[18], out int atMod) ? atMod : 0,
                    KKMod = int.TryParse(matches[17], out int kkMod) ? kkMod : 0,
                    AtReach = matches[14],
                    TpReach = matches[15],
                    LoadTime = int.TryParse(matches[18], out int loadTime) ? loadTime : 0
                };
                Range.Add(range);
                await Database.AddWeapon(range);
            }
        }

        private async Task AddRanged(int i, string talent, List<string> matches)
        {
            string name = talent.Replace(' ', '_').Replace(".", "");
            if (!matches[1].Equals(string.Empty))
            {
                var temp = new MeleeWeapon(
                    name,
                    matches[1],
                    int.TryParse(matches[10], out int weight) ? weight : 0,
                    matches[0].Split(':', StringSplitOptions.RemoveEmptyEntries).First(),
                    matches[11])
                {
                    INI = int.TryParse(matches[3], out int ini) ? ini : 0,
                    MW = matches[4],
                    TpKK = matches[2]
                };

                Weapons.Add(temp);
                await Database.AddWeapon(temp);
            }

            if (i > 18)
            {
                var range = new RangedWeapon(
                    name,
                    matches[13].Replace(' ', '+'),
                    int.TryParse(matches[10], out int weight) ? weight : 0,
                    matches[0].Split(':', StringSplitOptions.RemoveEmptyEntries).First(),
                    matches[11])
                {
                    AtMod = int.TryParse(matches[18], out int atMod) ? atMod : 0,
                    KKMod = int.TryParse(matches[17], out int kkMod) ? kkMod : 0,
                    AtReach = matches[14],
                    TpReach = matches[15],
                    LoadTime = int.TryParse(matches[18], out int loadTime) ? loadTime : 0
                };
                Range.Add(range);
                await Database.AddWeapon(range);
            }
        }
    }
}

