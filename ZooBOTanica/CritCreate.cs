using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ZooBOTanica
{
    using DSALib;
    using DSALib.Characters;

    public partial class CritCreateForm : Form
    {
        private DSALib.Characters.Critter critter;

        public CritCreateForm()
        {
            this.InitializeComponent();
            this.AllowDrop = true;
        }

        private void Load(string path)
        {
            this.critter = Critter.Load(path);

            this.AeEdit.Value = this.critter.Astralpunkte_Basis;
            this.AuEdit.Value = this.critter.Ausdauer_Basis;
            this.GWEdit.Value = this.critter.Gw;
            this.GsEdit.Value = this.critter.Gs;
            this.KoEdit.Value = this.critter.Ko;
            this.LeEdit.Value = this.critter.Lebenspunkte_Basis;
            this.MREdit.Value = this.critter.Mr;
            this.NameEdit.Text = this.critter.Name;
            //this.PAEdit.Value = this.critter.Pa;
            this.RSEdit.Value = this.critter.Rs;
            this.INIEdit.Text = this.critter.Ini;
            this.MeisterkommentarEdit.Text = this.critter.Comment;

            this.AttackList.Rows.Clear();

            foreach (var critterAttack in this.critter.CritterAttacks)
            {
                this.AttackList.Rows.Add(critterAttack.Name, critterAttack.At, critterAttack.Tp, critterAttack.Comment);
            }
        }
        
        private void CritCreateForm_DragDrop(object sender, DragEventArgs e)
        {
            this.Load(e.Data.GetData(DataFormats.Text).ToString());
        }

        private void LoadButton_Click(object sender, EventArgs e)
        {
            var dig = new OpenFileDialog();
            dig.CheckFileExists = true;
            dig.Multiselect = false;
            dig.Title = "Gespeicherten Gegner laden";
            dig.Filter = "*Json Dateien (*.json)|*.json";

            if (dig.ShowDialog() == DialogResult.OK)
            {
                this.Load(dig.FileName);
            }
        }

        private void SaveButton_Click(object sender, EventArgs e)
        {
            this.critter = new Critter();
            this.critter.Astralpunkte_Basis = (int)this.AeEdit.Value;
            this.critter.Ausdauer_Basis = (int)this.AuEdit.Value;
            this.critter.Gw = (int)this.GWEdit.Value;
            this.critter.Gs = (int)this.GsEdit.Value;
            this.critter.Ko = (int)this.KoEdit.Value;
            this.critter.Lebenspunkte_Basis = (int)this.LeEdit.Value;
            this.critter.Mr = (int)this.MREdit.Value;
            this.critter.Name = this.NameEdit.Text;
            //this.critter.Pa = (int)this.PAEdit.Value;
            this.critter.Rs = (int)this.RSEdit.Value;
            this.critter.Ini = this.INIEdit.Text;
            this.critter.Comment = this.MeisterkommentarEdit.Text;

            this.critter.CritterAttacks = new List<CritterAttack>();

            for (var index = 0; index < this.AttackList.Rows.Count -1; index++)
            {
                DataGridViewRow Row = this.AttackList.Rows[index];
                this.critter.CritterAttacks.Add(
                    new CritterAttack(
                        (Row.Cells[0].Value ?? "").ToString(),
                        Convert.ToInt32(Row.Cells[1].Value ?? 0),
                        (Row.Cells[2].Value ?? "").ToString(),
                        (Row.Cells[3].Value ?? "").ToString()));
            }

            this.critter.Save();
        }
    }
}

