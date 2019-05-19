using System;
using DSACore.Auxiliary;
using DSACore.Auxiliary.Calculator;
using DSACore.DSA_Game;
using DSACore.Models.Network;

namespace DSACore.Commands
{
    public class CommandHandler
    {
        public static CommandResponse ExecuteCommand(Command cmd)
        {
            var res = string.Empty;
            var type = ResponseType.Broadcast;
            switch (cmd.CmdIdentifier.ToLower())
            {
                case "addChar":
                    res = FileHandler.AddChar(cmd.CharId, cmd.CmdText);
                    break;
                case "held":
                case "wert":
                case "werte":
                case "char":
                    res = HeldList.ListAsync(cmd.CharId, cmd.CmdText);
                    break;
                case "help":
                case "man":
                case "hilfe":
                case "h":
                    res = Help.ShowHelp(cmd.CmdTexts.ToArray());
                    type = ResponseType.Caller;
                    break;
                case "le":
                case "leben":
                case "lp":
                    res = LE.LEAsync(cmd.CharId, cmd.CmdText);
                    break;
                case "ae":
                case "astral":
                case "asp":
                    res = AE.AEAsync(cmd.CharId, cmd.CmdText);
                    break;
                case "list":
                    res = List.ListAsync(cmd.CmdText);
                    type = ResponseType.Caller;
                    break;
                case "r":
                case "roll":
                    res = RandomMisc.Roll(cmd.CmdText + " " + cmd.Cmdmodifier);
                    break;
                case "solve":
                    res = new StringSolver(cmd.CmdText + cmd.Cmdmodifier).Solve().ToString();
                    break;
                case "npc":
                    res = NpcCommands.CreateNpc(cmd.CharId, cmd.CmdTexts, cmd.Cmdmodifier);
                    break;
            }

            if (res == string.Empty) res = Proben(cmd.Name, cmd.CmdIdentifier, cmd.CmdText, cmd.Cmdmodifier);
            if (res != string.Empty) return new CommandResponse(res, type);
            return new CommandResponse($"Kommando {cmd.CmdIdentifier} nicht gefunden", ResponseType.Error);
        }

        private static string Proben(string name, string command, string waffe, int erschwernis = 0)
        {
            var res = string.Empty;
            switch (command.ToLower())
            {
                case "f":
                case "fern":
                case "fernkampf":
                    res = CheckCommand(name, CommandTypes.Fernkampf, waffe, erschwernis);
                    break;
                case "t":
                case "ta":
                case "talent":
                case "talente":
                    res = CheckCommand(name, CommandTypes.Talent, waffe, erschwernis);
                    break;
                case "e":
                case "ei":
                case "eigenschaft":
                    res = CheckCommand(name, CommandTypes.Eigenschaft, waffe, erschwernis);
                    break;
                case "z":
                case "za":
                case "zauber":
                case "magie":
                case "m":
                    res = CheckCommand(name, CommandTypes.Talent, waffe, erschwernis);
                    break;
                case "a":
                case "at":
                case "an":
                case "angrif":
                case "angriff":
                    res = CheckCommand(name, CommandTypes.Angriff, waffe, erschwernis);
                    break;
                case "p":
                case "pa":
                case "parade":
                    res = CheckCommand(name, CommandTypes.Parade, waffe, erschwernis);
                    break;
            }

            return res;
        }

        private static string CheckCommand(string name, CommandTypes command, string waffe, int erschwernis = 0)
        {
            var chr = Dsa.GetCharacter(0);

            switch (command)
            {
                case CommandTypes.Talent:
                    return chr.TestTalent(waffe, erschwernis);
                case CommandTypes.Eigenschaft:
                    return chr.TestEigenschaft(waffe, erschwernis);
                case CommandTypes.Angriff:
                    return chr.Angriff(waffe, erschwernis);
                case CommandTypes.Parade:
                    return chr.Parade(waffe, erschwernis);
                case CommandTypes.Fernkampf:
                    return chr.Fernkampf(waffe, erschwernis);
                case CommandTypes.Zauber:
                    return chr.TestZauber(waffe, erschwernis);
            }

            return $"{name} verwendet {waffe}";

            throw new NotImplementedException("access char by id ore name and group id");
        }
    }
}