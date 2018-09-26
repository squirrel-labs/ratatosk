using System;
using DSACore.Auxiliary;
using DSALib.Characters;

namespace DSACore.Characters
{
    using System;

    using DSACore.Auxiliary;
    using DSACore.DSA_Game.Characters;

    public class Npc : Being, ICharacter
    {
        private readonly int mean, stDv;

        public Npc(string name, int mean, int stDv)
        {
            this.mean = mean;
            this.stDv = stDv;
            this.Name = name;
        }

        public string TestTalent(string talent, int tap = 3)
        {
            for (int i = 0; i <= 2; i++)
            {
                // foreach property, dice and tap 
                int temp = Dice.Roll();
                int eigenschaft = (int)Math.Round(RandomMisc.Random(this.stDv, this.mean));

                if (eigenschaft < temp)
                {
                    tap -= temp - eigenschaft;
                }
            }

            if (tap >= 0)
            {
                return $"{this.Name} vollführt {talent} erfolgreich";
            }


            return $"{this.Name} scheitert an {talent}";
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0)
        {
            int temp = Dice.Roll();
            int prop = (int)Math.Round(RandomMisc.Random(this.stDv, this.stDv));
            
            if (temp + erschwernis < prop)
            {
                return $"{this.Name} vollführt {eigenschaft} erfolgreich";
            }

            return $"{this.Name} scheitert an {eigenschaft}";
        }

        public string Angriff(string waffe, int erschwernis = 0)
        {
            int temp = Dice.Roll();

            if (temp == 1)
            {
                return $"{this.Name} greift kritisch mit {waffe} an";
            }

            if (temp < erschwernis)
            {
                return $"{this.Name} greift mit {waffe} an";
            }

            return $"{this.Name} haut mit {waffe} daneben";
        }

        public string Parade(string waffe, int erschwernis = 0)
        {
            int temp = Dice.Roll();

            if (temp == 1)
            {
                return $"{this.Name} pariert mit {waffe} meisterlich";
            }

            if (temp < erschwernis)
            {
                return $"{this.Name} pariert mit {waffe} an";
            }

            return $"{this.Name} schafft es nicht mit {waffe} zu parieren";
        }

        public string Fernkampf(string waffe, int erschwernis = 0)
        {
            int temp = Dice.Roll();

            if (temp == 1)
            {
                return $"{this.Name} trifft kritisch mit {waffe}";
            }

            if (temp < erschwernis)
            {
                return $"{this.Name} greift mit {waffe} an";
            }

            return $"{this.Name} schießt mit {waffe} daneben";
        }

        public string TestZauber(string zauber, int erschwernis)
        {
            return TestTalent(zauber, erschwernis);
        }
    }
}
