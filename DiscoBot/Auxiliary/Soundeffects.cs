using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot.Auxiliary
{
    using System.Runtime.CompilerServices;
    using System.Threading;

    using DiscoBot.Commands;

    public enum Sound
    {
        Bell,
        Ding,
        Nooo,
        Monterkill,
        Finish,
        Wrong,
        Magic
    }

    public static class SoundEffects
    {
        public static async Task Play(Sound s)
        {
            string url = string.Empty;
            int vol = 256;
            switch (s)
            {
                case Sound.Bell:
                case Sound.Ding:
                    url = "https://www.myinstants.com/media/sounds/boxing-bell.mp3";
                    break;
                case Sound.Finish:
                    url = "https://www.myinstants.com/media/sounds/finishhim.swf.mp3";
                    break; 
                case Sound.Magic:
                    url = "https://www.myinstants.com/media/sounds/dream-harp-sound-effect.mp3";
                    break;
                case Sound.Monterkill:
                    url = "https://www.myinstants.com/media/sounds/announcer_kill_monster_01.mp3";
                    break;
                case Sound.Nooo:
                    url = "https://www.myinstants.com/media/sounds/nooo.swf.mp3";
                    break;
                case Sound.Wrong:
                    // url = "https://www.myinstants.com/media/sounds/stupid_dum_03.mp3";
                    // vol = 10;
                    url = "https://www.myinstants.com/media/sounds/wrong-answer-sound-effect.mp3";
                    vol = 50;
                    break;
            }

            if (url != string.Empty)
            {
                // await Dsa.Service.SendAudioAsync(url, vol);
                await Voice.SendAsync(url, vol);
            }

            throw new Exception("Ton Existiert nicht");
        }
    }
}
