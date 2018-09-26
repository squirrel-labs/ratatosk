namespace DSACore.Auxiliary.Calculator
{
    using System;
    
    /// <summary>
    /// Provides an ISolvable class to save numbers. The class handles Argument checking and conversion from string to int. 
    /// </summary>
    public class Argument : ISolvable
    {
        private readonly int value;

        public Argument(string value)
        {
            // check whether the value given is an empty string
            if (string.IsNullOrEmpty(value)) 
            {
                throw new ArgumentException("Argument kann nicht mit einem leeren string instanziert werden. ", nameof(value));
            }

            if (!int.TryParse(value, out int result))
            {
                throw new ArgumentException($"Kann {value} nicht in Integer konvertieren");
            }

            this.value = result;
        }

        public int Solve()
        {
            return this.value;
        }

        public override string ToString()
        {
            return this.value.ToString();
        }
    }
}