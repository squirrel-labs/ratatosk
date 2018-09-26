using System;
using System.Collections.Generic;
using System.Linq;
using DSACorev.Auxiliary.Calculator;

namespace DSACore.Auxiliary.Calculator
{
    using System;
    using System.Collections.Generic;
    using System.Linq;

    /// <summary>
    /// The StringSolver divides the calculation string into operations and SubStringSolvers if the string contains parentheses 
    /// </summary>
    public class StringSolver : ISolvable
    {
        private readonly string input;
        private readonly List<object> arguments = new List<object>();

        public StringSolver(string input)
        {
            this.input = input;
        }

        public override string ToString()
        {
            return "(0+" + this.input.Replace(" ", string.Empty).ToLower() + ")";
        }

        public int Solve()
        {
            string workInput = "0+" + this.input.Replace(" ", string.Empty).ToLower();
            workInput = ExpandParentheses(workInput);
            
            // Create a List of the different parts of the calculation, e.g.:{"0", "+", "(5+6)", "d", "3"}.
            this.AtomizeOperations(workInput);

            // traverse the List in order of Operation to Create the binary operation tree .
            this.NestOperations();

            // the List now contains only the top operation node, witch can be solved recursively,
            return ((ISolvable)this.arguments.First()).Solve();
        }

        private static string GetInner(ref string input) // extract the inner bracket an remove the section from the input string
        {
            int depth = 0;
            for (var index = 1; index < input.Length; index++)
            {
                char c = input[index];
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

            return string.Empty;
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
            for (int i = 0; i < input.Length - 1; i++)
            {
                if (input[i + 1] == '(' && char.IsNumber(input[i]))
                {
                    input = input.Insert(i + 1, "*");
                }
            }

            for (int i = 1; i < input.Length; i++)
            {
                if (input[i - 1] == ')' && char.IsNumber(input[i]))
                {
                    input = input.Insert(i, "*");
                }
            }

            return input;
        }

        private void AtomizeOperations(string workInput)
        {
            for (var index = 0; index < workInput.Length; index++)
            {
                char c = workInput[index];

                if (char.IsNumber(c))
                {
                    // if char number, check if at end of string, else continue looping
                    if (index == workInput.Length - 1)
                    {
                        // if at end of string; add remaining number to arguments
                        this.arguments.Add(new Argument(workInput.Substring(0, index + 1)));
                    }

                    continue;
                }

                switch (c)
                {
                    case ')':
                        throw new ArgumentException($"Unmögliche Anordnung von Klammern");
                    case '(':
                        this.arguments.Add(new StringSolver(GetInner(ref workInput)));
                        index = -1;
                        break;
                    default:
                        if (index > 0)
                        {
                            this.arguments.Add(new Argument(workInput.Substring(0, index)));
                        }

                        this.arguments.Add(GetOps(c));
                        workInput = workInput.Remove(0, index + 1);
                        index = -1;
                        break;
                }
            }
        }

        private void NestOperations()
        {
            foreach (Ops currentOp in Enum.GetValues(typeof(Ops)))
            {
                // cycle through operators in operational order
                for (var index = 0; index < this.arguments.Count; index++)
                {
                    var arg = this.arguments[index];

                    if (arg.GetType() != typeof(Ops))
                    {
                        continue;
                    }

                    // arg is of type Ops
                    var op = (Ops)arg;

                    if (op != currentOp)
                    {
                        continue;
                    }

                    // arg describes the current operation
                    this.HandleSpecialFormatting(ref index, op); // Deal with special needs...

                    // replace the previous current and next Element in the List with one Operation object
                    var temp = new Operator((ISolvable)this.arguments[index - 1], (ISolvable)this.arguments[index + 1], op); 
                    this.arguments[index - 1] = temp;
                    this.arguments.RemoveRange(index, 2);
                    index--;
                }
            }
        }

        private void HandleSpecialFormatting(ref int index, Ops op)
        {
            var arg1 = this.arguments[index - 1];
            if (arg1.GetType() == typeof(Ops))
            {
                if (op == Ops.Dice)
                {
                    this.arguments.Insert(index++, new Argument("1")); // w6 -> 1w6
                }

                if (op == Ops.Subtract)
                {
                    this.arguments.Insert(index++, new Argument("0")); // +-3 -> +0-3
                }
            }

            var arg2 = this.arguments[index + 1]; // 3+-5 -> 3+(0-5)
            if (arg2.GetType() == typeof(Ops))
            {
                this.arguments[index + 1] = new Operator(new Argument("0"), (ISolvable)this.arguments[index + 2], (Ops)arg2);
                this.arguments.RemoveAt(index + 2);
            }
        }
    }
}