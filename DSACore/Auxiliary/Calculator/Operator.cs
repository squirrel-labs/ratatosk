using System;
using DSACorev.Auxiliary.Calculator;

namespace DSACore.Auxiliary.Calculator
{
    /// <summary>
    /// The Operator Class represents a binary operator with tow Arguments and an Operation type
    /// </summary>
    public class Operator : ISolvable
    {
        private readonly ISolvable arg1, arg2;

        public Operator(ISolvable arg1, ISolvable arg2, Ops operatorType)
        {
            this.arg1 = arg1;
            this.arg2 = arg2;
            this.OperatorType = operatorType;
        }

        public Ops OperatorType { get; set; }

        public int Solve()
        {
            int result;
            switch (this.OperatorType)
            {
                case Ops.Dice:
                    result = Dice.Roll(this.arg1.Solve(), this.arg2.Solve());
                    break;
                case Ops.Multiply:
                    result = this.arg1.Solve() * this.arg2.Solve();
                    break;
                case Ops.Add:
                    result = this.arg1.Solve() + this.arg2.Solve();
                    break;
                case Ops.Subtract:
                    result = this.arg1.Solve() - this.arg2.Solve();
                    break;
                default:
                    throw new ArgumentOutOfRangeException();
            }

            return result;
        }

        public override string ToString()
        {
            return $"({this.arg1} {this.OperatorType} {this.arg2})";
        }
    }
}
