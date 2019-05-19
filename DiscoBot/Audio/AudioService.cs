using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using Discord;
using Discord.Audio;

namespace DiscoBot.Audio
{
    public class AudioService
    {
        private readonly ConcurrentDictionary<ulong, IAudioClient> connectedChannels =
            new ConcurrentDictionary<ulong, IAudioClient>();

        public async Task JoinAudio(IGuild guild, IVoiceChannel target)
        {
            if (connectedChannels.TryGetValue(guild.Id, out var client)) return;

            if (target.Guild.Id != guild.Id) return;

            var audioClient = await target.ConnectAsync();

            if (connectedChannels.TryAdd(guild.Id, audioClient))
            {
                // If you add a method to log happenings from this service,
                // you can uncomment these commented lines to make use of that.
                //await Log(LogSeverity.Info, $"Connected to voice on {guild.Name}.");
            }
        }

        public async Task LeaveAudio(IGuild guild)
        {
            if (connectedChannels.TryRemove(guild.Id, out var client))
                await client.StopAsync();
            //await Log(LogSeverity.Info, $"Disconnected from voice on {guild.Name}.");
        }

        public async Task SendAudioAsync(IGuild guild, IMessageChannel channel, string path)
        {
            // Your task: Get a full path to the file if the value of 'path' is only a filename.
            if (!File.Exists(path) && false)
            {
                await channel.SendMessageAsync("File does not exist.");
                return;
            }

            if (connectedChannels.TryGetValue(guild.Id, out var client))
                //await Log(LogSeverity.Debug, $"Starting playback of {path} in {guild.Name}");
                using (var ffmpeg = CreateStream(path))
                using (var stream = client.CreatePCMStream(AudioApplication.Music))
                {
                    try
                    {
                        await ffmpeg.StandardOutput.BaseStream.CopyToAsync(stream);
                    }
                    finally
                    {
                        await stream.FlushAsync();
                    }
                }
        }

        public async Task SendAudioAsync(string path, int volume)
        {
            // Your task: Get a full path to the file if the value of 'path' is only a filename.
            if (!File.Exists(path) && false)
                //await channel.SendMessageAsync("File does not exist.");
                return;

            throw new NotImplementedException("get channel data from server");
            /*if (this.connectedChannels.TryGetValue())
            {
                //await Log(LogSeverity.Debug, $"Starting playback of {path} in {guild.Name}");
                using (var ffmpeg = this.CreateStream(path))
                using (var stream = client.CreatePCMStream(AudioApplication.Voice))
                {
                    try { await ffmpeg.StandardOutput.BaseStream.CopyToAsync(stream); }
                    finally { await stream.FlushAsync(); }
                }
            }*/
        }

        private static Process CreateStream(string path)
        {
            return Process.Start(new ProcessStartInfo
            {
                FileName = "ffmpeg.exe",
                Arguments = $"-hide_banner -loglevel panic -i \"{path}\" -ac 2 -f s16le -ar 48000 pipe:1",
                UseShellExecute = false,
                RedirectStandardOutput = true
            });
        }
    }
}