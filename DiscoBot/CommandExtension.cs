using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    using System.ComponentModel;
    using System.Threading;
    using Discord;
    using Discord.Commands;

    public static class CommandExtension
    {
        public static async Task ReplyTimed(this ModuleBase m, string message, TimeSpan time)
        {
            var token = message.GetHashCode();
            var send = m.Context.Channel.SendMessageAsync($"#{token}\n```xl\n{message}```", true);

            BackgroundWorker barInvoker = new BackgroundWorker();
            barInvoker.DoWork += delegate
                {
                    Thread.Sleep(time);
                    delete(token, m);
                };

            await send;
            barInvoker.RunWorkerAsync();
        }

        private static async void delete(int token, ModuleBase m)
        {
            var messagesAsync = m.Context.Channel.GetMessagesAsync(100);
            Task.WaitAll(messagesAsync.ToArray());
            var list = messagesAsync.ToEnumerable().ToList();
            var messages = new List<IMessage>();
            foreach (var task in list)
            {
                messages.AddRange(task.ToList());
            }

            await m.Context.Channel.DeleteMessagesAsync(messages.Where(x => x.Content.StartsWith($"#{token}\n") && x.Author.IsBot));
        }
    }
    
}
