using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Reflection;
using Discord;
using Discord.WebSocket;
using Discord.Commands;
using Microsoft.Extensions.DependencyInjection;


namespace DiscoBot
{
    public class Program
    {
        private CommandService commands;
        private DiscordSocketClient client;
        private IServiceProvider services;
        


        static void Main(string[] args) => new Program().Start().GetAwaiter().GetResult();

        public async Task Start()
        {
            var loading = DSA.Startup();
            client = new DiscordSocketClient();
            commands = new CommandService();

            string token = Properties.Settings.Default.Token;

            services = new ServiceCollection()
                    .BuildServiceProvider();
            AppDomain.CurrentDomain.ProcessExit += new EventHandler(OnProcessExit);

            await InstallCommands();

            await client.LoginAsync(TokenType.Bot, token);
            await client.StartAsync();

            await loading;
            await Task.Delay(-1);
        }

        public async Task InstallCommands()
        {
            // Hook the MessageReceived Event into our Command Handler
            client.MessageReceived += HandleCommand;
            // Discover all of the commands in this assembly and load them.
            await commands.AddModulesAsync(Assembly.GetEntryAssembly());
        }

        public async Task HandleCommand(SocketMessage messageParam)
        {
            // Don't process the command if it was a System Message
            var message = messageParam as SocketUserMessage;
            if (message == null) return;
            // Create a number to track where the prefix ends and the command begins
            int argPos = 0;
            // Determine if the message is a command, based on if it starts with '!' or a mention prefix
            if (!(message.HasCharPrefix('!', ref argPos) || message.HasMentionPrefix(client.CurrentUser, ref argPos))) return;
            // Create a Command Context
            var context = new CommandContext(client, message);
            // Execute the command. (result does not indicate a return value, 
            // rather an object stating if the command executed successfully)
            var result = await commands.ExecuteAsync(context, argPos, services);
            if (!result.IsSuccess)
                await context.Channel.SendMessageAsync(result.ErrorReason);
        }
        static void OnProcessExit(object sender, EventArgs e)
        {
            Console.WriteLine("I'm out of here");
            Voice.client.StopAsync();
        }
    }

}
