using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace DSACore.Models.Network
{
    public class CommandResponse
    {
        public CommandResponse(string message, ResponseType responseType = ResponseType.Broadcast)
        {
            this.message = message ?? throw new ArgumentNullException(nameof(message));
            ResponseType = responseType;
        }

        public string message { get; private set; }
        public ResponseType ResponseType { get; private set; }

        public override string ToString()
        {
            return message;
        }
    }

    public enum ResponseType
    {
        Broadcast,
        Caller,
        Error
    }
}