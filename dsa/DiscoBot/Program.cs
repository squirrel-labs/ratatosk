using System;
using System.IO;
using System.Linq;
using System.Net;
using System.Reflection;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;
using Discord.WebSocket;

namespace DiscoBot
{
    public class Program
    {
        private DiscordSocketClient client;
        private CommandService commands;
        private IServiceProvider services = null;

        public static void Main(string[] args)
        {
            new Program().StartAsync().GetAwaiter().GetResult();
        }

        public async Task StartAsync()
        {
            client = new DiscordSocketClient();
            commands = new CommandService();


            var token = File.ReadAllText("Token");
            //Properties.Settings.Default.Token;

            AppDomain.CurrentDomain.ProcessExit += OnProcessExit;

            await InstallCommandsAsync();

            await client.LoginAsync(TokenType.Bot, token);
            await client.StartAsync();

            await Task.Delay(-1);
        }

        public Task InstallCommandsAsync()
        {
            // Hook the MessageReceived Event into our Command Handler
            client.MessageReceived += HandleCommandAsync;

            // Discover all of the commands in this assembly and load them.
            return commands.AddModulesAsync(Assembly.GetEntryAssembly());
        }

        public async Task HandleCommandAsync(SocketMessage messageParam)
        {
            // Don't process the command if it was a System Message
            if (!(messageParam is SocketUserMessage message)) return;

            // Create a number to track where the prefix ends and the command begins
            var argPos = 0;

            // Determine if the message is a command, based on if it starts with '!' or a mention prefix
            if (!(message.HasCharPrefix('!', ref argPos) ||
                  message.HasMentionPrefix(client.CurrentUser, ref argPos))) return;


            // Create a Command Context
            var context = new CommandContext(client, message);

            // Execute the command. (result does not indicate a return value, 
            // rather an object stating if the command executed successfully)
            var result = await commands.ExecuteAsync(context, argPos, services);
            if (result.Error == CommandError.UnknownCommand)
                await context.Channel.SendMessageAsync(SendCommand(message.Author.Username, message.Content,
                    "https://kobert.dev/api/dsa/commands"));
            else if (!result.IsSuccess) await context.Channel.SendMessageAsync(result.ErrorReason);
        }

        private static string SendCommand(string name, string command, string url)
        {
            var httpWebRequest = (HttpWebRequest) WebRequest.Create(url);
            httpWebRequest.ContentType = "application/json";
            httpWebRequest.Method = "POST";

            using (var streamWriter = new StreamWriter(httpWebRequest.GetRequestStream()))
            {
                command = command.Remove(0, 1);
                var args = command.Split(new[] {' '}, StringSplitOptions.RemoveEmptyEntries);

                var content = string.Empty;
                if (args.Length > 1) content = "\"" + args.Skip(1).Aggregate((s, n) => s + "\", \"" + n) + "\"";

                var json = "{\"Name\":\"" + name + "\"," +
                           "\"CmdIdentifier\":\"" + args.First() + "\"," +
                           "\"CmdTexts\": [" + content + "] }";


                streamWriter.Write(json);
                streamWriter.Flush();
                streamWriter.Close();
            }

            var httpResponse = (HttpWebResponse) httpWebRequest.GetResponse();
            using (var streamReader = new StreamReader(httpResponse.GetResponseStream()))
            {
                return streamReader.ReadToEnd();
            }
        }

        private static void OnProcessExit(object sender, EventArgs e)
        {
            Console.WriteLine("I'm out of here");
        }
    }
}