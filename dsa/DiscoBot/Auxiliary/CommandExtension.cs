using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Net;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Discord;
using Discord.Commands;

namespace DiscoBot.Auxiliary
{
    public static class CommandExtension
    {
        private static WebClient _client;

        public static async Task ReplyTimedAsync(this ModuleBase m, string message, TimeSpan time)
        {
            var token = message.GetHashCode();
            var send = m.Context.Channel.SendMessageAsync($"#{token}\n```xl\n{message}```");

            var barInvoker = new BackgroundWorker();
            barInvoker.DoWork += delegate
            {
                Thread.Sleep(time);
                Delete(token, m);
            };

            await send;
            barInvoker.RunWorkerAsync();
        }

        private static void Delete(int token, ModuleBase m)
        {
            var messagesAsync = m.Context.Channel.GetMessagesAsync();
            Task.WaitAll(messagesAsync.ToArray());
            var list = messagesAsync.ToEnumerable().ToList();
            var messages = new List<IMessage>();
            foreach (var task in list) messages.AddRange(task.ToList());

            var test = messages.Where(x => x.Content.StartsWith($"#{token}\n") && x.Author.IsBot).Select(c => c);
            Task.WaitAll(test.Select(message => (message as IUserMessage)?.DeleteAsync()).ToArray());
        }

        public static async Task ReplyAsync(this ModuleBase m, IEnumerable<string> message, bool directMessage = false)
        {
            var sb = new StringBuilder();
            foreach (var re in message)
            {
                if (sb.Length + re.Length > 1798)
                {
                    if (directMessage)
                        await m.Context.User.SendMessageAsync("```xl\n" + sb + "\n```");
                    else
                        await m.Context.Channel.SendMessageAsync("```xl\n" + sb + "\n```");

                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            if (directMessage)
                await m.Context.User.SendMessageAsync("```xl\n" + sb + "\n```");
            else
                await m.Context.Channel.SendMessageAsync("```xl\n" + sb + "\n```");
        }

        public static async Task ReplyAsync(this ModuleBase m, IEnumerable<string> message, TimeSpan time)
        {
            var sb = new StringBuilder();
            foreach (var re in message)
            {
                if (sb.Length + re.Length > 1798)
                {
                    await m.ReplyTimedAsync(sb.ToString(), time);


                    sb.Clear();
                }

                sb.AppendLine(re);
            }

            await m.ReplyTimedAsync(sb.ToString(), TimeSpan.FromSeconds(90));
        }

        public static async Task SendWebFile(this IMessageChannel channel,
            string url = "https://i.imgur.com/0iHEycJ.png")
        {
            if (_client == null) _client = new WebClient();

            var stream = _client.OpenRead(url);
            await channel.SendFileAsync(stream, url.Split('/').Last());
        }
    }
}