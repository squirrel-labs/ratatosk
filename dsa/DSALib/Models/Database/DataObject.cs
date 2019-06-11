namespace DSALib.Models.Database
{
    public class DataObject : IDataObject
    {

        public override string ToString()
        {
            return Name;
        }

        public string Name { get; set; }
    }
}
