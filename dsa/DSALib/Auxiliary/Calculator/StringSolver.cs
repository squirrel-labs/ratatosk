using System;
using System.Collections.Generic;
using System.Linq;
using DSALibv.Auxiliary.Calculator;

namespace DSALib.Auxiliary.Calculator
{
    /// <summary>
    ///     The StringSolver divides the calculation string into operations and SubStringSolvers if the string contains
    ///     parentheses
    /// </summary>
    public class StringSolver : ISolvable
    {
        private readonly List<object> arguments = new List<object>();
        private readonly string input;

        public StringSolver(string input)
        {
            this.input = input;
        }

        public int Solve()
        {
            var workInput = "0+" + input.Replace(" ", string.Empty).ToLower();
            workInput = ExpandParentheses(workInput);

            // Create a List of the different parts of the calculation, e.g.:{"0", "+", "(5+6)", "d", "3"}.
            AtomizeOperations(workInput);

            // traverse the List in order of Operation to Create the binary operation tree .
            NestOperations();

            // the List now contains only the top operation node, witch can be solved recursively,
            return ((ISolvable) arguments.First()).Solve();
        }

        public override string ToString()
        {
            return "(0+" + input.Replace(" ", string.Empty).ToLower() + ")";
        }

        private static string
            GetInner(ref string input) // extract the inner bracket an remove the section from the input string
        {
            var depth = 0;
            for (var index = 1; index < input.Length; index++)
            {
                var c = input[index];
                switch (c)
                {
                    case '(':
                        depth++;
                        break;
                    case ')':
                        if (depth == 0)
                        {
                            var split = input.Substring(1, index - 1);
                            input = input.Substring(index + 1);
                            return split.Equals(string.Empty) ? "0" : split;
                        }
                        else
                        {
                            depth--;
                        }

                        break;
                }
            }

            throw new ArgumentException("Invalid brace sequence");
        }

        private static Ops GetOps(char c)
        {
            switch (c)
            {
                case 'd':
                case 'w':
                    return Ops.Dice;
                case '+':
                    return Ops.Add;
                case '-':
                    return Ops.Subtract;
                case '*':
                    return Ops.Multiply;
                default:
                    return Ops.Multiply;
            }
        }

        private static string ExpandParentheses(string input) // insert * between Parentheses and digits
        {
            for (var i = 0; i < input.Length - 1; i++)
                if (input[i + 1] == '(' && char.IsNumber(input[i]))
                    input = input.Insert(i + 1, "*");

            for (var i = 1; i < input.Length; i++)
                if (input[i - 1] == ')' && char.IsNumber(input[i]))
                    input = input.Insert(i, "*");

            return input;
        }

        private void AtomizeOperations(string workInput)
        {
            for (var index = 0; index < workInput.Length; index++)
            {
                var c = workInput[index];

                if (char.IsNumber(c))
                {
                    // if char number, check if at end of string, else continue looping
                    if (index == workInput.Length - 1)
                        // if at end of string; add remaining number to arguments
                        arguments.Add(new Argument(workInput.Substring(0, index + 1)));

                    continue;
                }

                switch (c)
                {
                    case ')':
                        throw new ArgumentException("Invalid brace sequence");
                    case '(':
                        arguments.Add(new StringSolver(GetInner(ref workInput)));
                        index = -1;
                        break;
                    default:
                        if (index > 0) arguments.Add(new Argument(workInput.Substring(0, index)));

                        arguments.Add(GetOps(c));
                        workInput = workInput.Remove(0, index + 1);
                        index = -1;
                        break;
                }
            }
        }

        private void NestOperations()
        {
            foreach (Ops currentOp in Enum.GetValues(typeof(Ops)))
                // cycle through operators in operational order
                for (var index = 0; index < arguments.Count; index++)
                {
                    var arg = arguments[index];

                    if (arg.GetType() != typeof(Ops)) continue;

                    // arg is of type Ops
                    var op = (Ops) arg;

                    if (op != currentOp) continue;

                    // arg describes the current operation
                    HandleSpecialFormatting(ref index, op); // Deal with special needs...

                    // replace the previous current and next Element in the List with one Operation object
                    var temp = new Operator((ISolvable) arguments[index - 1], (ISolvable) arguments[index + 1], op);
                    arguments[index - 1] = temp;
                    arguments.RemoveRange(index, 2);
                    index--;
                }
        }

        private void HandleSpecialFormatting(ref int index, Ops op)
        {
            var arg1 = arguments[index - 1];
            if (arg1.GetType() == typeof(Ops))
            {
                if (op == Ops.Dice) arguments.Insert(index++, new Argument("1")); // w6 -> 1w6

                if (op == Ops.Subtract) arguments.Insert(index++, new Argument("0")); // +-3 -> +0-3
            }

            var arg2 = arguments[index + 1]; // 3+-5 -> 3+(0-5)
            if (arg2.GetType() == typeof(Ops))
            {
                arguments[index + 1] = new Operator(new Argument("0"), (ISolvable) arguments[index + 2], (Ops) arg2);
                arguments.RemoveAt(index + 2);
            }
        }
    }
}