using System;
using System.Linq;

namespace DSACore.Auxiliary
{
    public static class Dice // roll it!
    {
        private static readonly System.Random Rnd = new System.Random();

        public static int Roll(int d = 20)
        {
            return Rnd.Next(d) + 1;
        }

        public static int Roll(string input)
        {
            var strings = input.ToLower().Split(new[] { 'w', 'd' }, 2, StringSplitOptions.RemoveEmptyEntries).ToList();
            int count = Convert.ToInt32(strings[0]);
            int d = Convert.ToInt32(strings[0]);

            if (strings.Count != 2)
            {
                throw new ArgumentException($"{input}: erfüllt nicht die Formatvogaben( Anzahl d Augenzahl)");
            }

            return Roll(count, d);
        }

        public static int Roll(int count, int d)
        {
            if (d <= 0)
            {
                return 0;
            }

            int sum = 0;
            for (int i = 0; i < Math.Abs(count); i++)
            {
                var roll = Dice.Roll(d);
                sum += roll;
            }

            sum *= Math.Abs(count) / count;

            return sum;
        }
    }
}
