using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.HttpsPolicy;
using Microsoft.AspNetCore.Mvc;
using DSACore.Hubs;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace DSACore
{
    public class Startup
    {
        public Startup(IConfiguration configuration)
        {
            Configuration = configuration;
        }

        public IConfiguration Configuration { get; }

        // This method gets called by the runtime. Use this method to add services to the container.
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddCors(options => options.AddPolicy("CorsPolicy",
                builder =>
                {
                    builder.AllowAnyOrigin()//.WithOrigins("https://dsa.truekuehli.de", "127.0.0.1")
                        .WithHeaders("Access-Control-Allow-Origin")
                        .AllowAnyHeader()
                        .AllowAnyMethod()
                        .AllowCredentials();
                }));
            /*
            services.AddCors(options =>
            {
                options.AddPolicy("AllowSpecificOrigin",
                    builder => builder.AllowAnyMethod().AllowAnyHeader().AllowAnyOrigin().AllowCredentials()/*WithOrigins("https://dsa.truekuehli.de")#1#);
            });

*/
            services.AddMvc().SetCompatibilityVersion(CompatibilityVersion.Version_2_1);

            services.AddSignalR();
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IHostingEnvironment env)
        {
            if (env.IsDevelopment()||true)
            {
                app.UseDeveloperExceptionPage();
            }
            else
            {
                app.UseHsts();
            }

            app.UseCors("CorsPolicy");

            app.UseSignalR(routes => { routes.MapHub<ChatHub>("/chatHub"); });

            
            //app.UseCors("AllowSpecificOrigin");
            //app.UseHttpsRedirection();
            app.UseMvc();
        }
    }
}
