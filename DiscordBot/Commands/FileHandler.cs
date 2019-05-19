using System;
using System.Linq;
using Discord.Commands;

namespace DiscordBot.Commands
{
    public class FileHandler : ModuleBase
    {
        //[Command("send"), Summary("fügt Helden hinzu")]
        public void AddChar()
        {
            var msg = Context.Message;
            if (msg.Attachments == null) throw new ArgumentException("Es wurde keine Datei angehängt");

            var attachments = msg.Attachments.ToList();

            if (!attachments.Any(x => x.Filename.EndsWith(".xml")))
                throw new ArgumentException("Es wurde kein xml Held mitgeschickt");

            foreach (var attachment in attachments.Where(x => x.Filename.EndsWith(".xml")))
                throw new NotImplementedException("send File to Server");
        }
    }
}