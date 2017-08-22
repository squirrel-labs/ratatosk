using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    public class imaginary
    {
        public double re;
        public double im;

        public imaginary(imaginary i)
        {
            this.re = i.re;
            this.im = i.im;
        }
        public imaginary(double r, double i)
        {
            this.re = r;
            this.im = i;
        }


        public void step(imaginary c)
        {
            double temp = re;
            re = (re * re) - (im * im) + c.re;
            im = (2.0 * temp * im + c.im);
        }
    }
}
