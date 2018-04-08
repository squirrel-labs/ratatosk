using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DiscoBot
{
    public interface ICharacter
    {
        string Name { get; set; }

        string TestTalent(string talent, int erschwernis = 0);

        string TestEigenschaft(string eigenschaft, int erschwernis = 0);

        string Angriff(string talent, int erschwernis = 0);

        string Parade(string talent, int erschwernis = 0);

        string Fernkampf(string talent, int erschwernis = 0);
    }
}
