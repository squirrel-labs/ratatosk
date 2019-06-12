using System;
using DSALib.Auxiliary;

namespace DSALib.Characters {
    public class Npc : Being, ICharacter {
        private readonly int mean, stDv;

        public Npc(string name, int mean, int stDv) {
            this.mean = mean;
            this.stDv = stDv;
            Name = name;
        }

        public string TestTalent(string talent, int tap = 3) {
            for (var i = 0; i <= 2; i++) {
                // foreach property, dice and tap 
                var temp = Dice.Roll();
                var eigenschaft = (int) Math.Round(RandomMisc.Random(stDv, mean));

                if (eigenschaft < temp) tap -= temp - eigenschaft;
            }

            if (tap >= 0) return $"{Name} vollführt {talent} erfolgreich";


            return $"{Name} scheitert an {talent}";
        }

        public string TestEigenschaft(string eigenschaft, int erschwernis = 0) {
            var temp = Dice.Roll();
            var prop = (int) Math.Round(RandomMisc.Random(stDv, stDv));

            if (temp + erschwernis < prop) return $"{Name} vollführt {eigenschaft} erfolgreich";

            return $"{Name} scheitert an {eigenschaft}";
        }

        public string Angriff(string waffe, int erschwernis = 0) {
            var temp = Dice.Roll();

            if (temp == 1) return $"{Name} greift kritisch mit {waffe} an";

            if (temp < erschwernis) return $"{Name} greift mit {waffe} an";

            return $"{Name} haut mit {waffe} daneben";
        }

        public string Parade(string waffe, int erschwernis = 0) {
            var temp = Dice.Roll();

            if (temp == 1) return $"{Name} pariert mit {waffe} meisterlich";

            if (temp < erschwernis) return $"{Name} pariert mit {waffe} an";

            return $"{Name} schafft es nicht mit {waffe} zu parieren";
        }

        public string Fernkampf(string waffe, int erschwernis = 0) {
            var temp = Dice.Roll();

            if (temp == 1) return $"{Name} trifft kritisch mit {waffe}";

            if (temp < erschwernis) return $"{Name} greift mit {waffe} an";

            return $"{Name} schießt mit {waffe} daneben";
        }

        public string TestZauber(string zauber, int erschwernis) {
            return TestTalent(zauber, erschwernis);
        }
    }
}