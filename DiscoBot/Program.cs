using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    class Program
    {
        static void Main(string[] args)
        {
            //new Form1();
            new System.Threading.Thread(Launch).Start();
            //MyBot Bot2 = new MyBot("MjU1NDM1MDUyMTg2MzM3Mjkw.Cydmeg.AV2aEAwrM9UHqOUnmmUXaC5TBm4");
        }
        public static void Launch()
        {
            new MyBot();
        }
    }
}
