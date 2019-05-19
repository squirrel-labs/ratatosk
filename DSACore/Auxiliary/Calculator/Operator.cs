using System;
using DSACorev.Auxiliary.Calculator;

namespace DSACore.Auxiliary.Calculator
{
    /// <summary>
    ///     The Operator Class represents a binary operator with tow Arguments and an Operation type
    /// </summary>
    public class Operator : ISolvable
    {
        private readonly ISolvable arg1, arg2;

        public Operator(ISolvable arg1, ISolvable arg2, Ops operatorType)
        {
            this.arg1 = arg1;
            this.arg2 = arg2;
            OperatorType = operatorType;
        }

        public Ops OperatorType { get; set; }

        public int Solve()
        {
            int result;
            switch (OperatorType)
            {
                case Ops.Dice:
                    result = Dice.Roll(arg1.Solve(), arg2.Solve());
                    break;
                case Ops.Multiply:
                    result = arg1.Solve() * arg2.Solve();
                    break;
                case Ops.Add:
                    result = arg1.Solve() + arg2.Solve();
                    break;
                case Ops.Subtract:
                    result = arg1.Solve() - arg2.Solve();
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }

            return result;
        }

        public override string ToString()
        {
            return $"({arg1} {OperatorType} {arg2})";
        }
    }
}