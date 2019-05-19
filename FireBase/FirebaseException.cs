namespace Firebase.Database
{
    using System;
    using System.Net;

    public class FirebaseException : Exception
    {
        public FirebaseException(string requestUrl, string requestData, string responseData, HttpStatusCode statusCode)
            : base(GenerateExceptionMessage(requestUrl, requestData, responseData))
        {
            RequestUrl = requestUrl;
            RequestData = requestData;
            ResponseData = responseData;
            StatusCode = statusCode;
        }

        public FirebaseException(string requestUrl, string requestData, string responseData, HttpStatusCode statusCode,
            Exception innerException)
            : base(GenerateExceptionMessage(requestUrl, requestData, responseData), innerException)
        {
            RequestUrl = requestUrl;
            RequestData = requestData;
            ResponseData = responseData;
            StatusCode = statusCode;
        }

        /// <summary>
        /// Post data passed to the authentication service.
        /// </summary>
        public string RequestData { get; }

        /// <summary>
        /// Original url of the request.
        /// </summary>
        public string RequestUrl { get; }

        /// <summary>
        /// Response from the authentication service.
        /// </summary>
        public string ResponseData { get; }

        /// <summary>
        /// Status code of the response.
        /// </summary>
        public HttpStatusCode StatusCode { get; }

        private static string GenerateExceptionMessage(string requestUrl, string requestData, string responseData)
        {
            return
                $"Exception occured while processing the request.\nUrl: {requestUrl}\nRequest Data: {requestData}\nResponse: {responseData}";
        }
    }
}