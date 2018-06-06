namespace DiscoBot.Commands
{
    using Discord.Commands;

    public class Man : ModuleBase
    //public class Man : ModuleBase
    {

        //Help for !LE command
        public string Man_LE()
        //public async Task Man_AE()
        {
            string res = "";
            res += "Use !LE to display, set, or change LE value\n\n";
            res += " !LE      Display values\n";
            res += " !LE 30   Set LE to 30\n";
            res += " !LE +5   Increment LE by 5 (up to the maximum)\n";
            res += " !LE ++5  Increment LE by 5 (ignoring the maximum)\n";
            res += " !LE -5   Decrease LE by 5\n";
            res += " \n";

            return res;
            //await this.ReplyAsync("```xl\n" + res + "\n```");
        }

        //Help for !AE command
        public string Man_AE()
        {
            string res = "";
            res += "Use !AE (or !Asp) to display, set, or change AE/Asp value\n\n";
            res += " !AE      Display values\n";
            res += " !AE 30   Set Asp to 30\n";
            res += " !AE +5   Increment Asp by 5 (up to the maximum)\n";
            res += " !AE ++5  Increment Asp by 5 (ignoring the maximum)\n";
            res += " !AE -5   Decrease Asp by 5 (down to 0)\n";
            res += " \n";

            return res;
         }

}
}
