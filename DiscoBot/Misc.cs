using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    public static class Misc
    {
        public static string Roll(string input)
        {
            int count = 1, d,mod=0;
            var Output = new StringBuilder();
            List<string> strings = input.Split('d').ToList();
            count = Convert.ToInt32(strings[0]);
            strings = strings[1].Split(' ').ToList();
            d = Convert.ToInt32(strings[0]);

            if (strings.Count > 0)
                mod = Convert.ToInt32(strings.Last());
            int sum = 0;
            for (int i = 0; i < count; i++)
            {
                var roll = dice.Roll(d);
                sum += roll;
                Output.Append(roll + " ");
            }
            if (count > 1)
                Output.Append("sum: " + (sum));

            return Output.ToString();
        }
    }
    public static class dice//roll it!
    {
        static System.Random rnd = new System.Random();
        public static int Roll(int  d=20)
        {
            return rnd.Next(1, d+1);
        }
    }
    public class Talent     //talent objekt
    {
        public string name, probe;
        public int value;
        public Talent(string name, string probe, int value) { this.name = name; this.probe = probe; this.value = value; }
        public string[] Test()      //turn XX/XX/XX into string[]{XX,XX,XX}
        {
            var temp = probe.Split('/');
            foreach (string s in temp)
                s.Replace("/", "");
            return temp;
        }

        public int CheckName(string quary)
        {
            if (quary.Equals(name))
                return 0;
            if (String.Compare(name, quary, StringComparison.InvariantCultureIgnoreCase) == 0)
                return 1;
            var subs = name.Split(' ','/');
            int score = subs.Count();
            foreach (String s in subs)
                if (String.Compare(name, quary, StringComparison.InvariantCultureIgnoreCase) == 0)
                    score--;
            if (score != subs.Count())
                return score+1;
            if (name.ToLowerInvariant().Contains(quary.ToLower()))
                return 3;

            return 100;
        }

    }
    public class Kampf
    {
        public string name;
        public int at, pa;
        public Kampf(string name, int at, int pa) { this.name = name; this.at = at; this.pa = pa; }
        void Test() { }
    }
    
}
