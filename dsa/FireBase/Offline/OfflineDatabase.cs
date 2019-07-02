using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using LiteDB;

namespace Firebase.Database.Offline {
    /// <summary>
    ///     The offline database.
    /// </summary>
    public class OfflineDatabase : IDictionary<string, OfflineEntry> {
        private readonly IDictionary<string, OfflineEntry> cache;
        private readonly LiteRepository db;

        /// <summary>
        ///     Initializes a new instance of the <see cref="OfflineDatabase" /> class.
        /// </summary>
        /// <param name="itemType"> The item type which is used to determine the database file name.  </param>
        /// <param name="filenameModifier"> Custom string which will get appended to the file name. </param>
        public OfflineDatabase(Type itemType, string filenameModifier) {
            var fullName = GetFileName(itemType.ToString());
            if (fullName.Length > 100) fullName = fullName.Substring(0, 100);

            var mapper = BsonMapper.Global;
            mapper.Entity<OfflineEntry>().Id(o => o.Key);

            var root = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
            var filename = fullName + filenameModifier + ".db";
            var path = Path.Combine(root, filename);
            db = new LiteRepository(new LiteDatabase(path, mapper));

            cache = db.Database.GetCollection<OfflineEntry>().FindAll()
                .ToDictionary(o => o.Key, o => o);
        }

        /// <summary>
        ///     Gets the number of elements contained in the <see cref="T:System.Collections.Generic.ICollection`1" />.
        /// </summary>
        /// <returns> The number of elements contained in the <see cref="T:System.Collections.Generic.ICollection`1" />. </returns>
        public int Count => cache.Count;

        /// <summary>
        ///     Gets a value indicating whether this is a read-only collection.
        /// </summary>
        public bool IsReadOnly => cache.IsReadOnly;

        /// <summary>
        ///     Gets an <see cref="T:System.Collections.Generic.ICollection`1" /> containing the keys of the
        ///     <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </summary>
        /// <returns>
        ///     An <see cref="T:System.Collections.Generic.ICollection`1" /> containing the keys of the object that
        ///     implements <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </returns>
        public ICollection<string> Keys => cache.Keys;

        /// <summary>
        ///     Gets an <see cref="T:System.Collections.Generic.ICollection`1" /> containing the values in the
        ///     <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </summary>
        /// <returns>
        ///     An <see cref="T:System.Collections.Generic.ICollection`1" /> containing the values in the object that
        ///     implements <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </returns>
        public ICollection<OfflineEntry> Values => cache.Values;

        /// <summary>
        ///     Gets or sets the element with the specified key.
        /// </summary>
        /// <param name="key">The key of the element to get or set.</param>
        /// <returns> The element with the specified key. </returns>
        public OfflineEntry this[string key] {
            get => cache[key];

            set {
                cache[key] = value;
                db.Upsert(value);
            }
        }

        /// <summary>
        ///     Returns an enumerator that iterates through the collection.
        /// </summary>
        /// <returns> An enumerator that can be used to iterate through the collection. </returns>
        public IEnumerator<KeyValuePair<string, OfflineEntry>> GetEnumerator() {
            return cache.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator() {
            return GetEnumerator();
        }

        /// <summary>
        ///     Adds an item to the <see cref="T:System.Collections.Generic.ICollection`1" />.
        /// </summary>
        /// <param name="item">The object to add to the <see cref="T:System.Collections.Generic.ICollection`1" />.</param>
        public void Add(KeyValuePair<string, OfflineEntry> item) {
            Add(item.Key, item.Value);
        }

        /// <summary>
        ///     Removes all items from the <see cref="T:System.Collections.Generic.ICollection`1" />.
        /// </summary>
        public void Clear() {
            cache.Clear();
            db.Delete<OfflineEntry>(LiteDB.Query.All());
        }

        /// <summary>
        ///     Determines whether the <see cref="T:System.Collections.Generic.ICollection`1" /> contains a specific value.
        /// </summary>
        /// <param name="item">The object to locate in the <see cref="T:System.Collections.Generic.ICollection`1" />.</param>
        /// <returns>
        ///     True if <paramref name="item" /> is found in the <see cref="T:System.Collections.Generic.ICollection`1" />;
        ///     otherwise, false.
        /// </returns>
        public bool Contains(KeyValuePair<string, OfflineEntry> item) {
            return ContainsKey(item.Key);
        }

        /// <summary>
        ///     Copies the elements of the <see cref="T:System.Collections.Generic.ICollection`1" /> to an
        ///     <see cref="T:System.Array" />, starting at a particular <see cref="T:System.Array" /> index.
        /// </summary>
        /// <param name="array">
        ///     The one-dimensional <see cref="T:System.Array" /> that is the destination of the elements copied
        ///     from <see cref="T:System.Collections.Generic.ICollection`1" />. The <see cref="T:System.Array" /> must have
        ///     zero-based indexing.
        /// </param>
        /// <param name="arrayIndex">The zero-based index in <paramref name="array" /> at which copying begins.</param>
        public void CopyTo(KeyValuePair<string, OfflineEntry>[] array, int arrayIndex) {
            cache.CopyTo(array, arrayIndex);
        }

        /// <summary>
        ///     Removes the first occurrence of a specific object from the
        ///     <see cref="T:System.Collections.Generic.ICollection`1" />.
        /// </summary>
        /// <param name="item">The object to remove from the <see cref="T:System.Collections.Generic.ICollection`1" />.</param>
        /// <returns>
        ///     True if <paramref name="item" /> was successfully removed from the
        ///     <see cref="T:System.Collections.Generic.ICollection`1" />; otherwise, false. This method also returns false if
        ///     <paramref name="item" /> is not found in the original <see cref="T:System.Collections.Generic.ICollection`1" />.
        /// </returns>
        public bool Remove(KeyValuePair<string, OfflineEntry> item) {
            return Remove(item.Key);
        }

        /// <summary>
        ///     Determines whether the <see cref="T:System.Collections.Generic.IDictionary`2" /> contains an element with the
        ///     specified key.
        /// </summary>
        /// <param name="key">The key to locate in the <see cref="T:System.Collections.Generic.IDictionary`2" />.</param>
        /// <returns>
        ///     True if the <see cref="T:System.Collections.Generic.IDictionary`2" /> contains an element with the key;
        ///     otherwise, false.
        /// </returns>
        public bool ContainsKey(string key) {
            return cache.ContainsKey(key);
        }

        /// <summary>
        ///     Adds an element with the provided key and value to the <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </summary>
        /// <param name="key">The object to use as the key of the element to add.</param>
        /// <param name="value">The object to use as the value of the element to add.</param>
        public void Add(string key, OfflineEntry value) {
            cache.Add(key, value);
            db.Insert(value);
        }

        /// <summary>
        ///     Removes the element with the specified key from the <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </summary>
        /// <param name="key">The key of the element to remove.</param>
        /// <returns>
        ///     True if the element is successfully removed; otherwise, false.  This method also returns false if
        ///     <paramref name="key" /> was not found in the original <see cref="T:System.Collections.Generic.IDictionary`2" />.
        /// </returns>
        public bool Remove(string key) {
            cache.Remove(key);
            return db.Delete<OfflineEntry>(key);
        }

        /// <summary>
        ///     Gets the value associated with the specified key.
        /// </summary>
        /// <param name="key">The key whose value to get.</param>
        /// <param name="value">
        ///     When this method returns, the value associated with the specified key, if the key is found;
        ///     otherwise, the default value for the type of the <paramref name="value" /> parameter. This parameter is passed
        ///     uninitialized.
        /// </param>
        /// <returns>
        ///     True if the object that implements <see cref="T:System.Collections.Generic.IDictionary`2" /> contains an
        ///     element with the specified key; otherwise, false.
        /// </returns>
        public bool TryGetValue(string key, out OfflineEntry value) {
            return cache.TryGetValue(key, out value);
        }

        private string GetFileName(string fileName) {
            var invalidChars = new[] {'`', '[', ',', '='};
            foreach (var c in invalidChars.Concat(Path.GetInvalidFileNameChars()).Distinct())
                fileName = fileName.Replace(c, '_');

            return fileName;
        }
    }
}