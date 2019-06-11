using System;

namespace DiscoBot.Auxiliary
{
    public static class Dice // roll it!
    {
        private static readonly Random Rnd = new Random();

        public static int Roll(int d = 20)
        {
            return Rnd.Next(d) + 1;
        }


        public static int Roll(int count, int d)
        {
            if (d <= 0) return 0;

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