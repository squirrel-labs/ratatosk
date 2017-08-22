using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Diagnostics;
using System.IO;

namespace DiscoBot
{
    class ServerControl
    {
        Process FTBProcess = new Process();
        ConsoleAppManager manager;

        public ServerControl()
        {
            manager = new ConsoleAppManager(DiscoBot.Properties.Settings.Default.ServerPaht + @"\ServerStart.bat");

            FTBProcess.StartInfo.FileName = /*@"C:\Program Files\Java\jdk1.8.0_101\jre\bin\java.exe";//*/DiscoBot.Properties.Settings.Default.ServerPaht + @"\ServerStart.bat";
            //FTBProcess.StartInfo.Arguments = @"-server -Xms512M -Xmx6G -XX:PermSize=256M -XX:+UseParNewGC -XX:+CMSIncrementalPacing -XX:+CMSClassUnloadingEnabled -XX:ParallelGCThreads=2 -XX:MinHeapFreeRatio=5 -XX:MaxHeapFreeRatio=10 -jar C:\Users\Dennis\Downloads\FTBBeyondServer\minecraft_server.1.10.2.jar nogui";
            FTBProcess.StartInfo.WorkingDirectory = /*@"C:\Program Files\Java\jdk1.8.0_101\jre\bin";*/Properties.Settings.Default.ServerPaht;
            

            
        }
        

        private void Refresh()
        {
            while(true)
                Console.WriteLine(FTBProcess.StandardOutput.ReadToEnd());
        }

        public void Start()
        {
            FTBProcess.Start();
            new System.Threading.Thread(Refresh).Start();

        }


        public void Command(string c)
        {
            FTBProcess.StandardInput.WriteLine(c);
        }

        public void Stop()
        {
            Process[] myProcesses;
            myProcesses = Process.GetProcessesByName("java");
            foreach (Process p in myProcesses)
                p.Kill();

            
        }
        
    }
}
