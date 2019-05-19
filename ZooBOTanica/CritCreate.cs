using System;
using System.Collections.Generic;
using System.Windows.Forms;
using DSALib;
using DSALib.Characters;

namespace ZooBOTanica
{
    public partial class CritCreateForm : Form
    {
        public Critter critter;

        public CritCreateForm()
        {
            InitializeComponent();
            AllowDrop = true;
        }

        public new void Load(string path)
        {
            critter = Critter.Load(path);

            AeEdit.Value = critter.Astralpunkte_Basis;
            AuEdit.Value = critter.Ausdauer_Basis;
            GWEdit.Value = critter.Gw;
            GsEdit.Value = critter.Gs;
            KoEdit.Value = critter.Ko;
            LeEdit.Value = critter.Lebenspunkte_Basis;
            MREdit.Value = critter.Mr;
            NameEdit.Text = critter.Name;
            //this.PAEdit.Value = this.critter.Pa;
            RSEdit.Value = critter.Rs;
            INIEdit.Text = critter.Ini;
            MeisterkommentarEdit.Text = critter.Comment;

            AttackList.Rows.Clear();

            foreach (var critterAttack in critter.CritterAttacks)
                AttackList.Rows.Add(critterAttack.Name, critterAttack.At, critterAttack.Tp, critterAttack.Comment);
        }

        public void CritCreateForm_DragDrop(object sender, DragEventArgs e)
        {
            Load(e.Data.GetData(DataFormats.Text).ToString());
        }

        public void LoadButton_Click(object sender, EventArgs e)
        {
            var dig = new OpenFileDialog
            {
                CheckFileExists = true,
                Multiselect = false,
                Title = "Gespeicherten Gegner laden",
                Filter = "*Json Dateien (*.json)|*.json"
            };

            if (dig.ShowDialog() == DialogResult.OK) Load(dig.FileName);
        }

        public void SaveButton_Click(object sender, EventArgs e)
        {
            critter = new Critter();
            critter.Astralpunkte_Basis = (int) AeEdit.Value;
            critter.Ausdauer_Basis = (int) AuEdit.Value;
            critter.Gw = (int) GWEdit.Value;
            critter.Gs = (int) GsEdit.Value;
            critter.Ko = (int) KoEdit.Value;
            critter.Lebenspunkte_Basis = (int) LeEdit.Value;
            critter.Mr = (int) MREdit.Value;
            critter.Name = NameEdit.Text;
            //this.critter.Pa = (int)this.PAEdit.Value;
            critter.Rs = (int) RSEdit.Value;
            critter.Ini = INIEdit.Text;
            critter.Comment = MeisterkommentarEdit.Text;

            critter.CritterAttacks = new List<CritterAttack>();

            for (var index = 0; index < AttackList.Rows.Count - 1; index++)
            {
                var Row = AttackList.Rows[index];
                critter.CritterAttacks.Add(
                    new CritterAttack(
                        (Row.Cells[0].Value ?? "").ToString(),
                        Convert.ToInt32(Row.Cells[1].Value ?? 0),
                        (Row.Cells[2].Value ?? "").ToString(),
                        (Row.Cells[3].Value ?? "").ToString()));
            }

            critter.Save();
        }
    }
}