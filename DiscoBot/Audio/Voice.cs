using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Threading.Tasks;
using DiscoBot.Auxiliary;
using Discord;
using Discord.Audio;
using Discord.Commands;

namespace DiscoBot.Audio
{
    public class Voice : ModuleBase
    {
        public static IAudioClient Client { get; set; }

        public static void Send(string path, int volume = 256)
        {
            if (Client == null) throw new NullReferenceException("Bot befindet sich nicht in einem Sprachchannel");

            // Create FFmpeg using the previous example
            var ffmpeg = CreateStream(path, volume);
            var output = ffmpeg.StandardOutput.BaseStream;
            var barInvoker = new BackgroundWorker();
            barInvoker.DoWork += delegate
            {
                var discord = Client.CreatePCMStream(AudioApplication.Music);
                output.CopyToAsync(discord);

                discord.FlushAsync();
            };

            barInvoker.RunWorkerAsync();
        }

        [Command("join", RunMode = RunMode.Async)]
        public async Task JoinChannelAsync(IVoiceChannel channel = null)
        {
            var msg = Context.Message;

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
//            Permissions.Test(this.Context, "Meister");

            if (Client != null)
            {
                await Client.StopAsync();
                Client = null;
            }
        }


        [Command("play", RunMode = RunMode.Async)]
        public async Task PlayAudioAsync(string path)
        {
            if (Client == null) await Context.Channel.SendMessageAsync("Erst Joinen!");

            //SoundEffects.Play(path);

            var sounds = Enum.GetValues(typeof(Sound));
            var soundList = new List<Sound>();
            foreach (var sound in sounds) soundList.Add((Sound) sound);

            var sc = new SpellCorrect();
        }

        private static Process CreateStream(string path, int vol = 256)
        {
            var ffmpeg = new ProcessStartInfo
            {
                FileName = "ffmpeg",
                Arguments = $"-i {path}  -ac 2 -f s16le -ar 48000 -ab 620000 -vol {vol} pipe:1",
                UseShellExecute = false,
                RedirectStandardOutput = true
            };
            return Process.Start(ffmpeg);
        }
    }
}