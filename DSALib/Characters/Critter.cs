using System;
using System.Collections.Generic;
using System.IO;
using DiscoBot.DSA_Game.Characters;
using DSALib.Models.Dsa;
using Newtonsoft.Json;

namespace DSALib.Characters
{
    public class Critter : Being, ICombatant
    {
        public CritterAttack lastAttack;

        public Critter(int gw, int gs, int rs, int mr, int ko, int pa, string ini, List<CritterAttack> critterAttacks)
        {
            Gw = gw;
            Gs = gs;
            Rs = rs;
            Mr = mr;
            Ko = ko;
            Pa = pa;
            Ini = ini;
            CritterAttacks = critterAttacks;
            lastAttack = CritterAttacks[new Random().Next(critterAttacks.Count)];
        }

        public Critter()
        {
        }

        public int Rs { get; set; }

        public int Mr { get; set; }

        public int Ko { get; set; }

        public int Pa { get; set; }

        public int Gs { get; set; }

        public int Gw { get; set; }

        public string Ini { get; set; }

        public string Comment { get; set; }

        public List<CritterAttack> CritterAttacks { get; set; }

        public string Angriff(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public string Parade(string talent, int erschwernis = 0)
        {
            throw new NotImplementedException();
        }

        public static Critter Load(string path)
        {
            try
            {
                return
                    JsonConvert.DeserializeObject<Critter>(
                        File.ReadAllText(path)); // Deserialize Data and create Session Object
            }
            catch (Exception e)
            {
                Console.WriteLine($"Laden von Save-File {path} fehlgeschlagen." + e);
                return null;
            }
        }

        public void Save(string path = @"..\..\Critters\")
        {
            try
            {
                File.WriteAllText(path + Name + ".json",
                    JsonConvert.SerializeObject(this,
                        Formatting.Indented)); // Deserialize Data and create CommandInfo Struct
            }
            catch (Exception e)
            {
                Console.WriteLine($"Speichern von Save-File {path} fehlgeschlagen." + e);
            }
        }
    }
}