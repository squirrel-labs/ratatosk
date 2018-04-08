using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    public static class Misc
    {
        private static readonly Random Rand = new Random();

        // use: 4w6 +4
        public static string Roll(string input) 
        {
            int count = 1, d ,mod=0;
            var Output = new StringBuilder();
            List<string> strings = input.Split('w','d').ToList();
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
                Output.Append("["+roll + "] ");
            }
            if (count > 1)
                Output.Append("sum: " + (sum));

            return Output.ToString();
        }

        public static double Random(double stdDev = 1, double mean = 0)
        {
            double u1 = Rand.NextDouble(); // uniform(0,1) random doubles
            double u2 = Rand.NextDouble();
            double randStdNormal = Math.Sqrt(-2.0 * Math.Log(u1)) *
                                   Math.Sin(2.0 * Math.PI * u2); // random normal(0,1)
            double randNormal =
                mean + stdDev * randStdNormal; // random normal(mean,stdDev^2)
            return randNormal;
        }
    }

    public class SpellCorrect : StringComparer
    {
        public override int Compare(string x, string y)
        {
            if (x.Equals(y))
                return 0;
            x=x.ToLower();
            y=y.ToLower();
            if (x.Equals(y))
                return 1;
            var subs = y.Split(' ', '/');
            int score = subs.Count();
            foreach (string s in subs)
            {
                if (s.Equals(x))
                {
                    score--;
                }
            }

            if (score < subs.Count())
            {
                return score + 1;
            }

            return 100000 - (int)(compare_exact(x, y) * 1000.0);
            /*if (y.Contains(x))
                return 6;*/
            
        }

        public override bool Equals(string x, string y)
        {
            throw new NotImplementedException();
        }

        public override int GetHashCode(string obj)
        {
            throw new NotImplementedException();
        }

        public double compare_exact(string s, string q)
        {
            int i, j;
            const double Match = 3.0;
            const double Gap = -2.0;
            const double Mismatch = -2.0;

            double decay = 0.0;

            double[,] matrix = new double[s.Length + 1, q.Length + 1];
            double max = 0.0;
            matrix[0, 0] = 0.0;
            
            for (i = 1; i < s.Length; i++)
            {
                matrix[i, 0] = 0.0;
            }

            for (i = 1; i < q.Length; i++)
            {
                matrix[0, i] = 0.0;
            }

            for (i = 1; i <= s.Length; i++)
            {
                for (j = 1; j <= q.Length; j++)
                {
                    decay = j / (double)(s.Length * 1000);
                    double add = s[i - 1] == q[j - 1] ? (Match - decay) : Mismatch;
                    double score = matrix[i - 1, j - 1] + add;

                    if (score < (matrix[i - 1, j] + Gap))
                    {
                        score = matrix[i - 1, j] + Gap;
                    }

                    if (score < (matrix[i, j - 1] + Gap))
                    {
                        score = matrix[i, j - 1] + Gap;
                    }

                    if (i > 1 && j > 1)
                    {
                        if (s[i - 1] == q[j - 2] && s[i - 2] == q[j - 1])
                        {
                            add = (3 / 2.0) * Match - decay;
                            if (score < matrix[i - 2, j - 2] + add)
                            {
                                score = matrix[i - 2, j - 2] + add;
                            }
                        }
                    }
                
                    if (score < 0)
                    {
                        score = 0;
                    }

                    if (max < score)
                    {
                        max = score;
                    }

                    matrix[i, j] = score;
                }
            }

            return max;
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

    public class Vorteil //talent objekt
    {
        public string name;
        public int value;

        public Vorteil(string name,  int value = 0)
        {
            this.name = name;
            this.value = value;
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
            var sc = (StringComparer)new SpellCorrect();
            return sc.Compare(quary, this.name);
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
