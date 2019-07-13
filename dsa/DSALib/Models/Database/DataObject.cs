namespace DSALib.Models.Database {
    public class DataObject : IDataObject {
        public string Name { get; set; }

        public override string ToString() {
            return Name;
        }
    }
}