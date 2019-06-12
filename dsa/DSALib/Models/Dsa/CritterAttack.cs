using DSALib.Models.Database;

namespace DSALib.Models.Dsa {
    public class CritterAttack : DataObject {
        public CritterAttack(string name, int at, string tp, string comment = "") {
            Name = name;
            At = at;
            Tp = tp;
            Comment = comment;
        }

        public int At { get; set; }

        public string Tp { get; set; }

        public string Comment { get; set; }
    }
}