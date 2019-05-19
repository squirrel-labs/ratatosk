using System;
using System.Linq;
using System.Text;

namespace DiscoBot.Auxiliary
{
    public static class RandomMisc
    {
        public static string Roll(string input)
        {
            var output = new StringBuilder();
            var strings = input.Split('w', 'd').ToList();
            var count = Convert.ToInt32(strings[0]);
            strings = strings[1].Split(' ').ToList();
            var d = Convert.ToInt32(strings[0]);

            if (strings.Count > 0)
            {
            }

            var sum = 0;
            for (var i = 0; i < count; i++)
            {
                var roll = Dice.Roll(d);
                sum += roll;
                output.Append("[" + roll + "] ");
            }

            if (strings.Count <= 1) return output.ToString();
            sum += Convert.ToInt32(strings[1]);
            output.Append("sum: " + sum);

            return output.ToString();
        }
    }
}