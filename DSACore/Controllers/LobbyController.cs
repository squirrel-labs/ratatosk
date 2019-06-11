using System;
using DSACore.Models.Network;
using DSALib.Commands;
using DSALib.Models.Network;
using Microsoft.AspNetCore.Mvc;

namespace DSACore.Controllers
{
    public class ScribbleController : Controller
    {
        [Route("[controller]")]
        // GET: api/<controller>
        [HttpGet]
        public string Get()
        {
            return "Usage: get /tokens/{Token}";
        }

        [HttpPost]
        public string Post([FromBody] Command cmd)
        {
            try
            {
                return CommandHandler.ExecuteCommand(cmd).message;
            }
            catch (Exception e)
            {
                return $"Ein Fehler ist aufgetreten: \n {e.Message}";
            }
        }
    }
}