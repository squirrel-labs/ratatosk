namespace DiscoBot.Commands
{
    using System;
    using System.Collections.Generic;
    using System.Diagnostics;
    using System.Linq;
    using System.Media;
    using System.Threading.Tasks;

    using DiscoBot.Auxiliary;

    using Discord;
    using Discord.Audio;
    using Discord.Commands;

    public class Voice : ModuleBase
    {
        public static IAudioClient Client { get; set; }

        public static async Task SendAsync(string path, int volume = 256)
        {
            // Create FFmpeg using the previous example
            var ffmpeg = CreateStream(path, volume);
            var output = ffmpeg.StandardOutput.BaseStream;
            var discord = Client.CreatePCMStream(AudioApplication.Music);
            await output.CopyToAsync(discord);
            await discord.FlushAsync();
        }

        [Command("join", RunMode = RunMode.Async)]
        public async Task JoinChannelAsync(IVoiceChannel channel = null)
        {
            var msg = this.Context.Message;

            // Get the audio channel
            channel = channel ?? (msg.Author as IGuildUser)?.VoiceChannel;
            if (channel == null)
            {
                await msg.Channel.SendMessageAsync(
                    "User must be in a voice channel, or a voice channel must be passed as an argument.");
                return;
            }

            // For the next step with transmitting audio, you would want to pass this Audio Client in to a service.
            var audioClient = await channel.ConnectAsync();
            Client = audioClient;
        }

        [Command("leave", RunMode = RunMode.Async)]
        public async Task LeaveChannelAsync(IVoiceChannel channel = null)
        {
            if (Client != null)
            {
                await Client.StopAsync();
                Client = null;
            }
        }

        [Command("play")]
        public Task PlayAudioAsync(string path)
        {
            if (Client == null)
            {
                return this.Context.Channel.SendMessageAsync("Erst Joinen!");
            }

            var sounds = Enum.GetValues(typeof(Sound));
            var soundList = new List<Sound>();
            foreach (var sound in sounds)
            {
                soundList.Add((Sound)sound);
            }

             var sc = new SpellCorrect();

            var tSound = soundList.OrderBy(x => sc.Compare(path, x.ToString())).First();

            if (sc.Compare(path, tSound.ToString()) > SpellCorrect.ErrorThreshold)
            {
                return SendAsync(path);
            }

            return SoundEffects.Play(tSound);
        }

        private static Process CreateStream(string path, int vol = 256)
        {
            var ffmpeg = new ProcessStartInfo
            {
                FileName = "ffmpeg",
                Arguments = $"-i {path}  -ac 2 -f s16le -ar 48000 -ab 620000 -vol {vol} pipe:1",
                UseShellExecute = false,
                RedirectStandardOutput = true,
            };
            return Process.Start(ffmpeg);
        }
    }
}
