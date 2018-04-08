using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    using System.Diagnostics;

    class Audio
    {
        private Process CreateStream(string path)
        {
            var ffmpeg = new ProcessStartInfo
                             {
                                 FileName = "ffmpeg",
                                 Arguments = $"-i {path} -ac 2 -f s16le -ar 48000 pipe:1",
                                 UseShellExecute = false,
                                 RedirectStandardOutput = true,
                             };
            return Process.Start(ffmpeg);
        }
    }

    
}
