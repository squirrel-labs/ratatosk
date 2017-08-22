using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Threading;
using System.Windows.Forms;

namespace DiscoBot
{

    public partial class Form1 : Form
    {
        PictureBox Box = new PictureBox();

        int farbtiefe = 255;
        Bitmap Canvas;
        int[,] plot;
        List<Bitmap> maps;
        bool working = false;

        int zoomque = 1;
        double zoomfactor = 10;
        double minx = -2, maxx = 1, miny = -1, maxy = 1;


        public Form1()
        {
            InitializeComponent();
            //this.Show();
            Box.Width = 500;
            Box.Height = 500;

            doStep();

        }
        public int Iterate(imaginary z)
        {

            imaginary c = new imaginary(z);
            z.re = 0; z.im = 0;
            int r=1;
            double temp;
            for (r = 0; r < farbtiefe; r++)//r=(int)Math.Pow(2, r)) 
            {
                z.step(c);
                temp = z.re * z.re + z.im * z.im;
                Console.WriteLine(r);
                if (temp > 4)
                    break;

                
            }
            

            return r;

        }

        private Color colorMap(int i)
        {
            /*/return Color.FromArgb(1, (int)2 * 2, (int)4 * 3);
            //i /= farbtiefe;
            int r = (int)(farbtiefe * (i / (double)max));
            //i /= farbtiefe;
            int b = (int)(farbtiefe * (i / (double)max));
            //i /= farbtiefe;
            int g = (int)(farbtiefe * (i / (double)max));
            */
            //i = 255;
            if (i != 0)
            { }
            var temp = new HSV();
            temp.h = i;
            temp.s = 1;
            temp.v = 1;

            return ColorFromHSL(temp);
        }

        


        public void doStep()
        {
            if (working == true)
                return;
            working = true;
            Canvas = new Bitmap(Box.Width, Box.Height);
            plot = new int[Box.Width, Box.Height];
            int processors = Environment.ProcessorCount;//get number of processors
            int perCore = (int)Box.Width / processors;//divide the cluster in equal parts
            int left = Box.Width - perCore * processors;//calc remainder
            List<Thread> threads = new List<Thread>();
            //maps = new List<Bitmap>();


            for (int i = 0; i < processors; i++)
            {
                int start = i * perCore;

                int stop = start + perCore;
                if (i == processors - 1)
                    stop += left - 1;
                int s = i;
                //maps.Add(new Bitmap(start-stop, Box.Height));
                threads.Add(new Thread(delegate () { paint(start, stop, s); }));
                threads.Last().Priority = ThreadPriority.BelowNormal;
                threads.Last().Start();
            }

            while (threads.Exists(x => x.IsAlive))
            {
                //Thread.Sleep(10);
                Application.DoEvents();
            }

            int max = 0;
            foreach (int i in plot)
                max = i > max ? i : max;

            int min = 0;
            foreach (int i in plot)
                min = i < min ? i : min;

            for (int x = 0; x < Box.Width; x++)
                for (int y = 0; y < Box.Height; y++)
                    Canvas.SetPixel(x, y, colorMap(plot[x, y]));

            //Box.Image = Canvas;//CombineBitmap(maps.ToArray());
            //Box.Refresh();
            Canvas.Save(@"C:\temp\temp.png");
            working = false;
        }

        private void paint(int start, int stop, int thread)
        {
            //int[,] c = new int[stop - start, Box.Height];
            for (int x = start; x < stop; x++)
                for (int y = 0; y < Box.Height; y++)
                {
                    plot[x, y] = Iterate(map(x, y));
                    Console.WriteLine(x);
                }

            /*int min = c.
            for (int x = start; x < stop; x++)
                for (int y = 0; y < Box.Height; y++)
                    maps[thread].SetPixel(x, y, colorMap(c[x-start,y],min,max));
                    */

        }


        public imaginary map(int x, int y)
        {
            double xl = Math.Abs(minx - maxx) * x / Box.Width + minx;
            double yl = Math.Abs(miny - maxy) * y / Box.Height + miny;


            return new imaginary(xl, yl);
        }

        private void zoom(object sender, MouseEventArgs e)
        {
            if (working == true)
            {
                if (e.Delta > 0)
                    zoomque++;
                else
                    zoomque--;
                return;
            }
            double zoom = Math.Abs((e.Delta / 120) * zoomfactor) * zoomque;

            double xl = Math.Abs(minx - maxx) * e.X / Box.Width + minx;
            double yl = Math.Abs(miny - maxy) * e.Y / Box.Height + miny;

            if (e.Delta > 0)
            {
                minx += Math.Abs(xl - minx) / zoom;
                miny += Math.Abs(yl - miny) / zoom;
                maxx -= Math.Abs(xl - maxx) / zoom;
                maxy -= Math.Abs(yl - maxy) / zoom;
            }
            else
            {
                minx -= Math.Abs(xl - maxx) / zoom;
                miny -= Math.Abs(yl - maxy) / zoom;
                maxx += Math.Abs(xl - minx) / zoom;
                maxy += Math.Abs(yl - miny) / zoom;
            }
            doStep();
            zoomque = 1;

        }



        private void Box_Click(object sender, EventArgs e)
        {
            doStep();


        }

        private void Form1_ResizeEnd(object sender, EventArgs e)
        {
            doStep();
        }

        public static System.Drawing.Bitmap CombineBitmap(Bitmap[] files)
        {
            //read all images into memory
            List<System.Drawing.Bitmap> images = new List<System.Drawing.Bitmap>();
            System.Drawing.Bitmap finalImage = null;

            try
            {
                int width = 0;
                int height = 0;

                foreach (Bitmap image in files)
                {
                    //create a Bitmap from the file and add it to the list
                    System.Drawing.Bitmap bitmap = new System.Drawing.Bitmap(image);

                    //update the size of the final bitmap
                    width += bitmap.Width;
                    height = bitmap.Height > height ? bitmap.Height : height;

                    images.Add(bitmap);
                }

                //create a bitmap to hold the combined image
                finalImage = new System.Drawing.Bitmap(width, height);

                //get a graphics object from the image so we can draw on it
                using (System.Drawing.Graphics g = System.Drawing.Graphics.FromImage(finalImage))
                {
                    //set background color
                    g.Clear(System.Drawing.Color.Black);

                    //go through each image and draw it on the final image
                    int offset = 0;
                    foreach (System.Drawing.Bitmap image in images)
                    {
                        g.DrawImage(image,
                          new System.Drawing.Rectangle(offset, 0, image.Width, image.Height));
                        offset += (int)image.Width / 2;
                    }
                }

                return finalImage;
            }
            catch (Exception)
            {
                if (finalImage != null)
                    finalImage.Dispose();
                //throw ex;
                throw;
            }
            finally
            {
                //clean up memory
                foreach (System.Drawing.Bitmap image in images)
                {
                    image.Dispose();
                }
            }
        }

        void HsvToRgb(double h, double S, double V, out int r, out int g, out int b)
        {
            // ######################################################################
            // T. Nathan Mundhenk
            // mundhenk@usc.edu
            // C/C++ Macro HSV to RGB

            double H = h;
            while (H < 0) { H += 360; };
            while (H >= 360) { H -= 360; };
            double R, G, B;
            if (V <= 0)
            { R = G = B = 0; }
            else if (S <= 0)
            {
                R = G = B = V;
            }
            else
            {
                double hf = H / 60.0;
                int i = (int)Math.Floor(hf);
                double f = hf - i;
                double pv = V * (1 - S);
                double qv = V * (1 - S * f);
                double tv = V * (1 - S * (1 - f));
                switch (i)
                {

                    // Red is the dominant color

                    case 0:
                        R = V;
                        G = tv;
                        B = pv;
                        break;

                    // Green is the dominant color

                    case 1:
                        R = qv;
                        G = V;
                        B = pv;
                        break;
                    case 2:
                        R = pv;
                        G = V;
                        B = tv;
                        break;

                    // Blue is the dominant color

                    case 3:
                        R = pv;
                        G = qv;
                        B = V;
                        break;
                    case 4:
                        R = tv;
                        G = pv;
                        B = V;
                        break;

                    // Red is the dominant color

                    case 5:
                        R = V;
                        G = pv;
                        B = qv;
                        break;

                    // Just in case we overshoot on our math by a little, we put these here. Since its a switch it won't slow us down at all to put these here.

                    case 6:
                        R = V;
                        G = tv;
                        B = pv;
                        break;
                    case -1:
                        R = V;
                        G = pv;
                        B = qv;
                        break;

                    // The color is not defined, we should throw an error.

                    default:
                        //LFATAL("i Value error in Pixel conversion, Value is %d", i);
                        R = G = B = V; // Just pretend its black/white
                        break;
                }
            }
            r = Clamp((int)(R * 255.0));
            g = Clamp((int)(G * 255.0));
            b = Clamp((int)(B * 255.0));
        }

        /// <summary>
        /// Clamp a value to 0-255
        /// </summary>
        int Clamp(int i)
        {
            if (i < 0) return 0;
            if (i > 255) return 255;
            return i;
        }


        Color SetHue(Color oldColor)
        {
            var temp = new HSV();
            temp.h = oldColor.GetHue();
            temp.s = oldColor.GetSaturation();
            return ColorFromHSL(temp);
        }
        public struct HSV { public  float h; public  float s; public float v; }

        // the Color Converter
        static public Color ColorFromHSL(HSV hsl)
        {
            if (hsl.s == 0)
            { int L = (int)hsl.v; return Color.FromArgb(255, L, L, L); }

            double min, max, h;
            h = hsl.h / 360d;

            max = hsl.v < 0.5d ? hsl.v * (1 + hsl.s) : (hsl.v + hsl.s) - (hsl.v * hsl.s);
            min = (hsl.v * 2d) - max;


            Color c = Color.FromArgb(255, (int)(255 * RGBChannelFromHue(min, max, h + 1 / 3d)),
                                          (int)(255 * RGBChannelFromHue(min, max, h)),
                                          (int)(255 * RGBChannelFromHue(min, max, h - 1 / 3d)));



            return c;
        }

        static double RGBChannelFromHue(double m1, double m2, double h)
        {
            h = (h + 1d) % 1d;
            if (h < 0) h += 1;
            if (h * 6 < 1) return m1 + (m2 - m1) * 6 * h;
            else if (h * 2 < 1) return m2;
            else if (h * 3 < 2) return m1 + (m2 - m1) * 6 * (2d / 3d - h);
            else return m1;

        }
    }
    
}
