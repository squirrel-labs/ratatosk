using System;

namespace DSALib.Models.Network
{
    public class CommandResponse
    {
        public CommandResponse(string message, ResponseType responseType = ResponseType.Broadcast)
        {
            this.message = message ?? throw new ArgumentNullException(nameof(message));
            ResponseType = responseType;
        }

        public string message { get; }
        public ResponseType ResponseType { get; }

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