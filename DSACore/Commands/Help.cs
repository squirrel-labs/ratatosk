using System.Linq;
using DSACore.Auxiliary;
using DSACore.DSA_Game.Save;

namespace DSACore.Commands
{
    public class Help
    {
        //public static List<CommandInfo> Commands { get; } = new List<CommandInfo>();


        public static string Get_Specific_Help(string command)
        {
            // return command specific help
            var com = Properties.CommandInfos
                .OrderBy(x => SpellCorrect.CompareEasy(x.Name, command.ToLower())).First(); // get best fit command
            return com.GetDescription();
        }

        public static string Get_Generic_Help()
        {
            var res = "";
            foreach (var com in Properties.CommandInfos)
            {
                var first_column_width = 8;
                res += ("!" + com.Name + ": ").AddSpaces(first_column_width) + com.Brief;

                if (com.Description.Length > 1)
                    res += "\n" + "".AddSpaces(first_column_width) + "(\"!man " + com.Name +
                           "\" gibt genauere Informationen)";

                res += "\n\n";
            }

            return res;
        }

        public static string ShowHelp(params string[] commandList)
        {
            var command = "";
            if (commandList.Length > 0) command = commandList.Aggregate((s, c) => s + " " + c);

            if (command.Equals(string.Empty)) // return generic Help
            {
                var res = Get_Generic_Help();

                return res;
            }


            return Get_Specific_Help(command);
        }
    }
}