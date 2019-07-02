using System.IO;
using System.Text;

namespace Firebase.Database.Streaming {
    /// <summary>
    ///     When a regular <see cref="StreamReader" /> is used in a UWP app its <see cref="StreamReader.ReadLine" /> method
    ///     tends to take a long
    ///     time for data larger then 2 KB. This extremly simple implementation of <see cref="TextReader" /> can be used
    ///     instead to boost performance
    ///     in your UWP app. Use <see cref="FirebaseOptions" /> to inject an instance of this class into your
    ///     <see cref="FirebaseClient" />.
    /// </summary>
    public class NonBlockingStreamReader : TextReader {
        private const int DefaultBufferSize = 16000;
        private readonly byte[] buffer;
        private readonly int bufferSize;

        private readonly Stream stream;

        private string cachedData;

        public NonBlockingStreamReader(Stream stream, int bufferSize = DefaultBufferSize) {
            this.stream = stream;
            this.bufferSize = bufferSize;
            buffer = new byte[bufferSize];

            cachedData = string.Empty;
        }

        public override string ReadLine() {
            var currentString = TryGetNewLine();

            while (currentString == null) {
                var read = stream.Read(buffer, 0, bufferSize);
                var str = Encoding.UTF8.GetString(buffer, 0, read);

                cachedData += str;
                currentString = TryGetNewLine();
            }

            return currentString;
        }

        private string TryGetNewLine() {
            var newLine = cachedData.IndexOf('\n');

            if (newLine >= 0) {
                var r = cachedData.Substring(0, newLine + 1);
                cachedData = cachedData.Remove(0, r.Length);
                return r.Trim();
            }

            return null;
        }
    }
}