using System;

namespace DSALib.Auxiliary
{
    public class SpellCorrect
    {
        public const double ErrorThreshold =  1 / 3.0;
        private const double Match = 3.0;
        private const double Gap = -1.5;
        private const double Mismatch = -2.0;

        public static double Compare(string s, string q)
        {
            s = s.ToLower();
            q = q.ToLower();

            int i, j;
            
            var matrix = new double[s.Length + 1, q.Length + 1];
            var max = 0.0;
            matrix[0, 0] = 0.0;

            for (i = 1; i < s.Length; i++)
                matrix[i, 0] = i * Gap;

            for (i = 1; i < q.Length; i++) matrix[0, i] = 0.0;


            for (i = 1; i <= s.Length; i++)
                for (j = 1; j <= q.Length; j++)
                {
                    double decay = j / (s.Length * 1000.0);
                    var add = s[i - 1] == q[j - 1] ? Match - decay : Mismatch;
                    var score = matrix[i - 1, j - 1] + add;

                    if (score < matrix[i - 1, j] + Gap) score = matrix[i - 1, j] + Gap;

                    if (score < matrix[i, j - 1] + Gap) score = matrix[i, j - 1] + Gap;

                    if (i > 1 && j > 1)
                        if (s[i - 1] == q[j - 2] && s[i - 2] == q[j - 1])
                        {
                            add = 3 / 2.0 * Match - decay;
                            if (score < matrix[i - 2, j - 2] + add) score = matrix[i - 2, j - 2] + add;
                        }

                    if (max < score && i == s.Length) max = score;

                    matrix[i, j] = score;
                }

            return max;
        }

        public static bool IsMatch(string s1, string s2)
        {
            var score = Compare(s1, s2);
            return score > ErrorThreshold * s1.Length;
        }
    }
}