namespace DiscoBot.Auxiliary
{
    using System;
    using System.Diagnostics;
    using System.Linq;

    public class SpellCorrect : StringComparer
    {
        public const int ErrorThreshold = 94100;

        public override int Compare(string x, string y)
        {
            if (string.IsNullOrEmpty(x))
            {
                throw new ArgumentException("message", nameof(x));
            }

            if (string.IsNullOrEmpty(y))
            {
                throw new ArgumentException("message", nameof(y));
            }

            if (x.Equals(y))
            {
                return 0;
            }

            x = x.ToLower();
            y = y.ToLower();
            if (x.Equals(y))
            {
                return 1;
            }

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

            return 100000 - (int)(this.CompareExact(x, y) * 1000.0);
            /*if (y.Contains(x))
                return 6;*/
        }

        public override bool Equals(string x, string y)
        {
            Debug.Assert(x != null, nameof(x) + " != null");
            return x.Equals(y);
        }

        public override int GetHashCode(string obj)
        {
            throw new NotImplementedException();
        }

        public double CompareExact(string s, string q)
        {

            s = s.ToLower();
            q = q.ToLower();

            int i, j;
            const double Match = 3.0;
            const double Gap = -2.0;
            const double Mismatch = -2.0;

            double decay;

            double[,] matrix = new double[s.Length + 1, q.Length + 1];
            double max = 0.0;
            matrix[0, 0] = 0.0;
            
            for (i = 1; i < s.Length; i++)
            {
               //  matrix[i, 0] = 0.0;
                matrix[i, 0] = i * Gap;
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
                
                  //  if (score < 0)
                  //  {
                  //      score = 0;
                  // }

                    if (max < score && i == s.Length)
                    {
                        max = score;
                    }

                    matrix[i, j] = score;
                }
            }

            return max;
        }
    }
}
