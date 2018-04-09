namespace DiscoBot.Auxiliary
{
    public static class Dice // roll it!
    {
        private static readonly System.Random Rnd = new System.Random();

        public static int Roll(int d = 20)
        {
            return Rnd.Next(d) + 1;
        }
    }
}
