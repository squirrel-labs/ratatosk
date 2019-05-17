using System;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;
using Microsoft.CodeAnalysis.CSharp.Syntax;

namespace DSACore.Controllers
{
    [Route("lobby/[controller]")]
    [ApiController]
    public class TokensController : Controller
    {
        
        // GET
        [HttpGet("{token}")]
        public async Task<ActionResult<string>> Get(int token)
        {

            if (!Hubs.Users.Tokens.Exists(x => x.GetHashCode() == token))
            {
                return NotFound();
            }

            var group = Hubs.Users.Tokens.Find(x => x.GetHashCode() == token);
            return Ok(group);
        }
    }
}