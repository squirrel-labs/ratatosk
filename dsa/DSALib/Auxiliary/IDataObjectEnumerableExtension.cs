using System;
using System.Collections.Generic;
using System.Linq;
using DSALib.Auxiliary;
using DSALib.Models.Database;

namespace DSACore.Auxiliary {
    public static class DataObjectEnumerableExtension {
        public static IDataObject Match(this IEnumerable<IDataObject> dataObjects, string name) {
            return (dataObjects as IOrderedEnumerable<IDataObject> ?? throw new InvalidOperationException())
                .OrderBy(x => SpellCorrect.Compare(name, x.Name)).Last();
        }

        public static bool TryMatch(this IEnumerable<IDataObject> dataObjects, out IDataObject data, string name) {
            data = (dataObjects as IOrderedEnumerable<IDataObject> ?? throw new InvalidOperationException())
                .OrderBy(x => SpellCorrect.Compare(name, x.Name)).Last();

            return SpellCorrect.IsMatch(name, data.Name);
        }
    }
}