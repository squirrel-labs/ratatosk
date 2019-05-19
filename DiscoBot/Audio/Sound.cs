namespace DiscoBot.Audio
{
    public class Sound
    {
        public Sound(string name, string url, int volume)
        {
            Name = name;
            Url = url;
            Volume = volume;
        }

        public string Name { get; }

        public string Url { get; }

        public int Volume { get; }
    }
}