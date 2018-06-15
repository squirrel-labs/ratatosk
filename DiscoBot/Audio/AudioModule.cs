using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Threading.Tasks;

using DiscoBot;
using DiscoBot.Audio;
using DiscoBot.Auxiliary;
using DiscoBot.Commands;

using Discord;
using Discord.Commands;

namespace DiscoBot.Audio
{
    using DiscoBot.DSA_Game;

    public class AudioModule : ModuleBase
    {
        // Scroll down further for the AudioService.
        // Like, way down
        private readonly AudioService _service;

        // Remember to add an instance of the AudioService
        // to your IServiceCollection when you initialize your bot
        public AudioModule(AudioService service)
        {
            _service = service;
            Dsa.Service = service;
        }

        // You *MUST* mark these commands with 'RunMode.Async'
        // otherwise the bot will not respond until the Task times out.
        [Command("_join", RunMode = RunMode.Async)]
        public async Task JoinCmd()
        {
            await _service.JoinAudio(Context.Guild, (Context.User as IVoiceState).VoiceChannel);
        }

        // Remember to add preconditions to your commands,
        // this is merely the minimal amount necessary.
        // Adding more commands of your own is also encouraged.
        [Command("_leave", RunMode = RunMode.Async)]
        public async Task LeaveCmd()
        {
            await _service.LeaveAudio(Context.Guild);
        }

        [Command("_play", RunMode = RunMode.Async)]
        public async Task PlayCmd([Remainder] string song)
        {
            if (Dsa.GeneralContext == null)
            {
                Dsa.GeneralContext = this.Context;
            }

            var sounds = Enum.GetValues(typeof(Sound));
            var soundList = new List<Sound>();
            foreach (var sound in sounds)
            {
                soundList.Add((Sound)sound);
            }

            var sc = new SpellCorrect();

            var tSound = soundList.OrderBy(x => sc.Compare(song, x.ToString())).First();

            if (sc.Compare(song, tSound.ToString()) > SpellCorrect.ErrorThreshold)
            {
                await _service.SendAudioAsync(Context.Guild, Context.Channel, song);
            }

            SoundEffects.Play(song);
        }
    }
}