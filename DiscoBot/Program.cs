using System;
using System.Reflection;
using System.Threading.Tasks;

using Discord;
using Discord.Commands;
using Discord.WebSocket;

using Microsoft.Extensions.DependencyInjection;

namespace DiscoBot
{
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

            string token = Properties.Settings.Default.Token;

            this.services = new ServiceCollection().AddSingleton(new AudioService())
                    .BuildServiceProvider();
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
            if (!result.IsSuccess)
            {
                await context.Channel.SendMessageAsync(result.ErrorReason);
            }
        }

        private static void OnProcessExit(object sender, EventArgs e)
        {
            Console.WriteLine("I'm out of here");
            Voice.Client.StopAsync();
        }
    }
}
