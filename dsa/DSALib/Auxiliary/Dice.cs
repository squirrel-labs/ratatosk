using System;
using System.Linq;

namespace DSALib.Auxiliary
{
    public static class Dice // roll it!
    {
        private static readonly Random Rnd = new Random();

        public static int Roll(int d = 20)
        {
            return Rnd.Next(d) + 1;
        }

        public static int Roll(string input)
        {
            var strings = input.ToLower().Split(new[] {'w', 'd'}, 2, StringSplitOptions.RemoveEmptyEntries).ToList();
            

            if (strings.Count != 2)
                throw new ArgumentException($"{input}: does not satisfy the format requirements( dice count (d|w) die size)");

            var count = Convert.ToInt32(strings[0]);
            var d = Convert.ToInt32(strings[0]);

            return Roll(count, d);
        }

        public static int Roll(int count, int d)
        {
            if (d <= 0 || count <= 0) return 0;

            var sum = 0;
            for (var i = 0; i < Math.Abs(count); i++)
            {
                var roll = Roll(d);
                sum += roll;
            }

            sum *= Math.Abs(count) / count;

            return sum;
        }
    }
}