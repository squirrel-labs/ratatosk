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
            if (!int.TryParse(token, out var inttoken))
            {
                return BadRequest("The token has to be a 32 bit unsigned integer");
            }

            if (!Hubs.Users.Tokens.Exists(x => x.GetHashCode() == inttoken))
            {
                return NotFound();
            }

            var group = Hubs.Users.Tokens.Find(x => x.GetHashCode() == inttoken);
            return Ok(group);
        }
    }
}
