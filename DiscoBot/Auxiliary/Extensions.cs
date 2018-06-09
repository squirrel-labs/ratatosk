namespace DiscoBot.Auxiliary
{
    public static class StringExtension
    {
        //This mehod extends string. It adds spaces until a fixed length is reached.
        //If the original string is already longer, it is returner unmodified.
        public static string AddSpaces(this string str, int length)
        {
            string temp = str;
        for(int i = str.Length; i < length; i++)
            {
                temp += " ";
            }
            return temp;
        }
    }
}
