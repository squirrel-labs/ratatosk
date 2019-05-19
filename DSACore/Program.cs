using DSACore.DSA_Game;
using DSACore.FireBase;
using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Hosting;

namespace DSACore
{
    public class Program
    {
        public static void Main(string[] args)
        {
            Database.GetGroup(0).Wait();
            Dsa.Startup();
            CreateWebHostBuilder(args).Build().Run();
        }

        public static IWebHostBuilder CreateWebHostBuilder(string[] args)
        {
            return WebHost.CreateDefaultBuilder(args)
                .UseStartup<Startup>()
                .UseUrls("http://0.0.0.0:5000");
        }
    }
}