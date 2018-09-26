namespace DSACore.Auxiliary
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
    


        //This mehod extends string. 
        //It adds spaces at the HEAD of a string until a fixed length is reached.
        //If the original string is already longer, it is returner unmodified.
        public static string AddSpacesAtHead(this string str, int length)
        {
            string temp = "";
            for (int i = str.Length; i < length; i++)
            {
                temp += " ";
            }
            return temp + str;
        }
    }

}
