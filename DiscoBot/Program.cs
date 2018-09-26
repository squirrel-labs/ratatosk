using System;
using System.Linq;
using System.Net;
using System.Reflection;
using System.Threading.Tasks;

using Discord;
using Discord.Commands;
using Discord.WebSocket;

using Microsoft.Extensions.DependencyInjection;

namespace DiscoBot
{
    using System.IO;

    using DiscoBot.Audio;
    using DiscoBot.DSA_Game;

    public class Program
    {
        private CommandService commands;
        private DiscordSocketClient client;
        private IServiceProvider services;

        public static void Main(string[] args) => new Program().StartAsync().GetAwaiter().GetResult();

        public async Task StartAsync()
        {
            Dsa.Startup();
            
            this.client = new DiscordSocketClient();
            this.commands = new CommandService();



            string token = File.ReadAllText("Token");
            //Properties.Settings.Default.Token;
            
            AppDomain.CurrentDomain.ProcessExit += OnProcessExit;

            await this.InstallCommandsAsync();

            await this.client.LoginAsync(TokenType.Bot, token);
            await this.client.StartAsync();
            
            await Task.Delay(-1);
        }

        public Task InstallCommandsAsync()
        {
            // Hook the MessageReceived Event into our Command Handler
            this.client.MessageReceived += this.HandleCommandAsync;
            
            // Discover all of the commands in this assembly and load them.
            return this.commands.AddModulesAsync(Assembly.GetEntryAssembly());
        }

        public async Task HandleCommandAsync(SocketMessage messageParam)
        {
            // Don't process the command if it was a System Message
            if (!(messageParam is SocketUserMessage message))
            {
                return;
            }

            // Create a number to track where the prefix ends and the command begins
            int argPos = 0;

            // Determine if the message is a command, based on if it starts with '!' or a mention prefix
            if (!(message.HasCharPrefix('!', ref argPos) || message.HasMentionPrefix(this.client.CurrentUser, ref argPos)))
            {
                return; 
            }

            
            // Create a Command Context
            var context = new CommandContext(this.client, message);
            
            // Execute the command. (result does not indicate a return value, 
            // rather an object stating if the command executed successfully)
            var result = await this.commands.ExecuteAsync(context, argPos, this.services);
            if (result.Error == CommandError.UnknownCommand)
            {
                await context.Channel.SendMessageAsync(SendCommand(message.Author.Username, message.Content, "https://localhost:44365/api/Commands"));
            }
            else if (!result.IsSuccess)
            {
                await context.Channel.SendMessageAsync(result.ErrorReason);
            }
        }

        private string SendCommand(string name, string command, string url)
        {
            var httpWebRequest = (HttpWebRequest)WebRequest.Create(url);
            httpWebRequest.ContentType = "application/json";
            httpWebRequest.Method = "POST";

            using (var streamWriter = new StreamWriter(httpWebRequest.GetRequestStream()))
            {
                command = command.Remove(0,1); 
                var args = command.Split(new []{' '}, StringSplitOptions.RemoveEmptyEntries);

                string content = string.Empty;
                if (args.Length > 1)
                {
                    content = "\"" + args.Skip(1).Aggregate((s, n) => ( s + "\", \"" + n)) + "\"";
                }

                string json = "{\"Name\":\"" + name + "\"," +
                              "\"CmdIdentifier\":\"" + args.First() + "\"," +
                              "\"CmdTexts\": ["+ content+"] }";
            

                streamWriter.Write(json);
                streamWriter.Flush();
                streamWriter.Close();
            }

            var httpResponse = (HttpWebResponse)httpWebRequest.GetResponse();
            using (var streamReader = new StreamReader(httpResponse.GetResponseStream()))
            {
                return streamReader.ReadToEnd();
            }
        }

        private static void OnProcessExit(object sender, EventArgs e)
        {
            Console.WriteLine("I'm out of here");
            Voice.Client.StopAsync();
        }
    }
}
