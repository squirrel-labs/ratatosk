using Discord;
using Discord.Commands;

using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Drawing;
using System.Windows;

namespace DiscoBot
{
    class MyBot
    {
        DiscordClient discord;
        CommandService commands;
        private String token;
        ServerControl FTB = new ServerControl();

        public MyBot(string token = "Mjk0NTU0MDU4Nzg4NzAwMTYx.C7XGwQ.VwCAM10lDmwUe01NhBvDKNbd17I")
        {
            this.token = token;

            discord = new DiscordClient(x =>
            {
                x.LogLevel = LogSeverity.Info;
                x.LogHandler = Log;
            });

            discord.UsingCommands(x =>
            {
                x.PrefixChar = '!';
                x.AllowMentionPrefix = true;
            });

            commands = discord.GetService<CommandService>();
            Mandelbrot();
            Server();
            DSA();

            discord.ExecuteAndWait(async () =>
            {
                await discord.Connect(token, TokenType.Bot);
            });
        }

        private void Mandelbrot()
        {
            commands.CreateCommand("mandelbrot")
                .Do(async (e) =>
                {
                    //await e.Channel.SendMessage("!hallo");
                    
                    await e.Channel.SendFile(@"C:\temp\temp.png");
                    

                });
        }
        private void Server()
        {
            
            commands.CreateCommand("start")
                .Do(async (e) =>
                {
                    await  e.Channel.SendMessage("Server wird gestartet");

                    FTB.Start();

                });
            commands.CreateCommand("stop")
                .Do(async (e) =>
                {
                    await e.Channel.SendMessage("Server wird gestoppt");

                    //FTB.Stop();
                });
            commands.CreateCommand("/")
                .Parameter("command",ParameterType.Required)
                .Parameter("value",ParameterType.Multiple)
                .Do(async (e) =>
                {
                    await e.Channel.SendMessage("Command wird ausgeführt");

                    FTB.Command(e.GetArg("command")+" "+e.GetArg("value"));
                });

            commands.CreateCommand("restart")
                .Do(async (e) =>
                {
                    await e.Channel.SendMessage("Server wird neu gestartet");

                    FTB.Stop();
                });
        
    }
        private void DSA()
        {
            commands.CreateCommand("wer ist Schuld?")
                .Do(async (e) =>
                {
                    await e.Channel.SendMessage(e.Channel.Users.ToArray()[new Random().Next(0,4)].ToString());

                    FTB.Stop();
                });
        }

        private void Log(object sender, LogMessageEventArgs e)
        {
            Console.WriteLine(e.Message);
        }
    }
}
