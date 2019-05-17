using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using DSACore.Models;
using DSACore.Models.Network;
using Microsoft.AspNetCore.Mvc;

// For more information on enabling Web API for empty projects, visit https://go.microsoft.com/fwlink/?LinkID=397860

namespace DSACore.Controllers
{
    [Route("dsa/[controller]")]
    public class CommandsController : Controller
    {
        // GET: api/<controller>
        [HttpGet]
        public string Get()
        {
            return "Usage: post the command to execute";
        }

        // GET api/<controller>/5
        /*[HttpGet("{id}")]
        public string Get(int id)
        {
            return "value";
        }*/

        // POST api/<controller>/Felis
        [HttpPost]
        public string Post([FromBody]Command cmd)
        {
            try
            {
                return Commands.CommandHandler.ExecuteCommand(cmd).message;
            }
            catch (Exception e)
            {
                return $"Ein Fehler ist aufgetreten: \n {e.Message}";
            }
            
        }

/*

        // PUT api/<controller>/5
        [HttpPut("{id}")]
        public void Put(int id, [FromBody]string value)
        {
        }

        // DELETE api/<controller>/5
        [HttpDelete("{id}")]
        public void Delete(int id)
        {
        }*/
    }
}
