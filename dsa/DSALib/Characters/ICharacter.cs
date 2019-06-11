using DiscoBot.DSA_Game.Characters;

namespace DSALib.Characters
{
    public interface ICharacter : ICombatant
    {
        string TestTalent(string talent, int erschwernis = 0);

        string TestEigenschaft(string eigenschaft, int erschwernis = 0);

        string Fernkampf(string talent, int erschwernis = 0);

        string TestZauber(string waffe, int erschwernis);
    }
}