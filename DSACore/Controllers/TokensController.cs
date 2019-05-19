using DSACore.Hubs;
using Microsoft.AspNetCore.Mvc;

namespace DSACore.Controllers
{
    [Route("lobby/[controller]")]
    [ApiController]
    public class TokensController : Controller
    {
        // GET
        [HttpGet("{token}")]
        public ActionResult<string> Get(string token)
        {
            if (!int.TryParse(token, out var intToken))
                return BadRequest("The token has to be a 32 bit unsigned integer");

            if (intToken == 42) return Ok("Scribble");
            
            if (!Users.Tokens.Exists(x => x.GetHashCode() == intToken)) return NotFound();

            var group = Users.Tokens.Find(x => x.GetHashCode() == intToken);
            return Ok(group.Group);
        }
    }
}