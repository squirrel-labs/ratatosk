namespace DiscoBot.DSA_Game.Characters
{
    public interface ICharacter
    {
        string Name { get; set; }

        int Lebenspunkte { get; set; }

        //int Ausdauer { get; set; }

        int Astralpunkte { get; set; }

        //int Karmapunkte { get; set; }

        string TestTalent(string talent, int erschwernis = 0);

        string TestEigenschaft(string eigenschaft, int erschwernis = 0);

        string Angriff(string talent, int erschwernis = 0);

        string Parade(string talent, int erschwernis = 0);

        string Fernkampf(string talent, int erschwernis = 0);

        string TestZauber(string waffe, int erschwernis);
    }
}
