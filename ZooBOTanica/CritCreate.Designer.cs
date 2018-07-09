namespace ZooBOTanica
{
    partial class CritCreateForm
    {
        /// <summary>
        /// Erforderliche Designervariable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Verwendete Ressourcen bereinigen.
        /// </summary>
        /// <param name="disposing">True, wenn verwaltete Ressourcen gelöscht werden sollen; andernfalls False.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Vom Windows Form-Designer generierter Code

        /// <summary>
        /// Erforderliche Methode für die Designerunterstützung.
        /// Der Inhalt der Methode darf nicht mit dem Code-Editor geändert werden.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(CritCreateForm));
            this.NameLabel = new System.Windows.Forms.Label();
            this.NameEdit = new System.Windows.Forms.TextBox();
            this.LeLabel = new System.Windows.Forms.Label();
            this.LeEdit = new System.Windows.Forms.NumericUpDown();
            this.GrundwerteGroup = new System.Windows.Forms.GroupBox();
            this.KoLabel = new System.Windows.Forms.Label();
            this.KoEdit = new System.Windows.Forms.NumericUpDown();
            this.AeLabel = new System.Windows.Forms.Label();
            this.AeEdit = new System.Windows.Forms.NumericUpDown();
            this.AuLabel = new System.Windows.Forms.Label();
            this.AuEdit = new System.Windows.Forms.NumericUpDown();
            this.VerteidugungGroup = new System.Windows.Forms.GroupBox();
            this.PaEdit = new System.Windows.Forms.NumericUpDown();
            this.PaLabel = new System.Windows.Forms.Label();
            this.MRLabel = new System.Windows.Forms.Label();
            this.MREdit = new System.Windows.Forms.NumericUpDown();
            this.RSLAbel = new System.Windows.Forms.Label();
            this.RSEdit = new System.Windows.Forms.NumericUpDown();
            this.SecondGroup = new System.Windows.Forms.GroupBox();
            this.INIEdit = new System.Windows.Forms.TextBox();
            this.GWLabel = new System.Windows.Forms.Label();
            this.GWEdit = new System.Windows.Forms.NumericUpDown();
            this.INILabel = new System.Windows.Forms.Label();
            this.GsLabel = new System.Windows.Forms.Label();
            this.GsEdit = new System.Windows.Forms.NumericUpDown();
            this.AttackGroup = new System.Windows.Forms.GroupBox();
            this.AttackList = new System.Windows.Forms.DataGridView();
            this.NameCollum = new System.Windows.Forms.DataGridViewTextBoxColumn();
            this.ATCollum = new System.Windows.Forms.DataGridViewTextBoxColumn();
            this.TPCollum = new System.Windows.Forms.DataGridViewTextBoxColumn();
            this.KommentarCollum = new System.Windows.Forms.DataGridViewTextBoxColumn();
            this.MeisterkommentarEdit = new System.Windows.Forms.TextBox();
            this.MeisterkommentarLabel = new System.Windows.Forms.Label();
            this.SaveButton = new System.Windows.Forms.Button();
            this.LoadButton = new System.Windows.Forms.Button();
            ((System.ComponentModel.ISupportInitialize)(this.LeEdit)).BeginInit();
            this.GrundwerteGroup.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.KoEdit)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.AeEdit)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.AuEdit)).BeginInit();
            this.VerteidugungGroup.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.PaEdit)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.MREdit)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.RSEdit)).BeginInit();
            this.SecondGroup.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.GWEdit)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.GsEdit)).BeginInit();
            this.AttackGroup.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.AttackList)).BeginInit();
            this.SuspendLayout();
            // 
            // NameLabel
            // 
            resources.ApplyResources(this.NameLabel, "NameLabel");
            this.NameLabel.Name = "NameLabel";
            // 
            // NameEdit
            // 
            this.NameEdit.ForeColor = System.Drawing.SystemColors.WindowText;
            resources.ApplyResources(this.NameEdit, "NameEdit");
            this.NameEdit.Name = "NameEdit";
            // 
            // LeLabel
            // 
            resources.ApplyResources(this.LeLabel, "LeLabel");
            this.LeLabel.Name = "LeLabel";
            // 
            // LeEdit
            // 
            resources.ApplyResources(this.LeEdit, "LeEdit");
            this.LeEdit.Name = "LeEdit";
            this.LeEdit.Value = new decimal(new int[] {
            30,
            0,
            0,
            0});
            // 
            // GrundwerteGroup
            // 
            this.GrundwerteGroup.Controls.Add(this.KoLabel);
            this.GrundwerteGroup.Controls.Add(this.KoEdit);
            this.GrundwerteGroup.Controls.Add(this.AeLabel);
            this.GrundwerteGroup.Controls.Add(this.AeEdit);
            this.GrundwerteGroup.Controls.Add(this.AuLabel);
            this.GrundwerteGroup.Controls.Add(this.AuEdit);
            resources.ApplyResources(this.GrundwerteGroup, "GrundwerteGroup");
            this.GrundwerteGroup.Name = "GrundwerteGroup";
            this.GrundwerteGroup.TabStop = false;
            // 
            // KoLabel
            // 
            resources.ApplyResources(this.KoLabel, "KoLabel");
            this.KoLabel.Name = "KoLabel";
            // 
            // KoEdit
            // 
            resources.ApplyResources(this.KoEdit, "KoEdit");
            this.KoEdit.Name = "KoEdit";
            this.KoEdit.Value = new decimal(new int[] {
            10,
            0,
            0,
            0});
            // 
            // AeLabel
            // 
            resources.ApplyResources(this.AeLabel, "AeLabel");
            this.AeLabel.Name = "AeLabel";
            // 
            // AeEdit
            // 
            resources.ApplyResources(this.AeEdit, "AeEdit");
            this.AeEdit.Name = "AeEdit";
            // 
            // AuLabel
            // 
            resources.ApplyResources(this.AuLabel, "AuLabel");
            this.AuLabel.Name = "AuLabel";
            // 
            // AuEdit
            // 
            resources.ApplyResources(this.AuEdit, "AuEdit");
            this.AuEdit.Name = "AuEdit";
            this.AuEdit.Value = new decimal(new int[] {
            30,
            0,
            0,
            0});
            // 
            // VerteidugungGroup
            // 
            this.VerteidugungGroup.Controls.Add(this.PaEdit);
            this.VerteidugungGroup.Controls.Add(this.PaLabel);
            this.VerteidugungGroup.Controls.Add(this.MRLabel);
            this.VerteidugungGroup.Controls.Add(this.MREdit);
            this.VerteidugungGroup.Controls.Add(this.RSLAbel);
            this.VerteidugungGroup.Controls.Add(this.RSEdit);
            resources.ApplyResources(this.VerteidugungGroup, "VerteidugungGroup");
            this.VerteidugungGroup.Name = "VerteidugungGroup";
            this.VerteidugungGroup.TabStop = false;
            // 
            // PaEdit
            // 
            resources.ApplyResources(this.PaEdit, "PaEdit");
            this.PaEdit.Name = "PaEdit";
            // 
            // PaLabel
            // 
            resources.ApplyResources(this.PaLabel, "PaLabel");
            this.PaLabel.Name = "PaLabel";
            // 
            // MRLabel
            // 
            resources.ApplyResources(this.MRLabel, "MRLabel");
            this.MRLabel.Name = "MRLabel";
            // 
            // MREdit
            // 
            resources.ApplyResources(this.MREdit, "MREdit");
            this.MREdit.Name = "MREdit";
            this.MREdit.Value = new decimal(new int[] {
            5,
            0,
            0,
            0});
            // 
            // RSLAbel
            // 
            resources.ApplyResources(this.RSLAbel, "RSLAbel");
            this.RSLAbel.Name = "RSLAbel";
            // 
            // RSEdit
            // 
            resources.ApplyResources(this.RSEdit, "RSEdit");
            this.RSEdit.Name = "RSEdit";
            // 
            // SecondGroup
            // 
            this.SecondGroup.Controls.Add(this.INIEdit);
            this.SecondGroup.Controls.Add(this.GWLabel);
            this.SecondGroup.Controls.Add(this.GWEdit);
            this.SecondGroup.Controls.Add(this.INILabel);
            this.SecondGroup.Controls.Add(this.GsLabel);
            this.SecondGroup.Controls.Add(this.GsEdit);
            resources.ApplyResources(this.SecondGroup, "SecondGroup");
            this.SecondGroup.Name = "SecondGroup";
            this.SecondGroup.TabStop = false;
            // 
            // INIEdit
            // 
            this.INIEdit.CharacterCasing = System.Windows.Forms.CharacterCasing.Lower;
            resources.ApplyResources(this.INIEdit, "INIEdit");
            this.INIEdit.Name = "INIEdit";
            // 
            // GWLabel
            // 
            resources.ApplyResources(this.GWLabel, "GWLabel");
            this.GWLabel.Name = "GWLabel";
            // 
            // GWEdit
            // 
            resources.ApplyResources(this.GWEdit, "GWEdit");
            this.GWEdit.Name = "GWEdit";
            this.GWEdit.Value = new decimal(new int[] {
            3,
            0,
            0,
            0});
            // 
            // INILabel
            // 
            resources.ApplyResources(this.INILabel, "INILabel");
            this.INILabel.Name = "INILabel";
            // 
            // GsLabel
            // 
            resources.ApplyResources(this.GsLabel, "GsLabel");
            this.GsLabel.Name = "GsLabel";
            // 
            // GsEdit
            // 
            resources.ApplyResources(this.GsEdit, "GsEdit");
            this.GsEdit.Name = "GsEdit";
            this.GsEdit.Value = new decimal(new int[] {
            8,
            0,
            0,
            0});
            // 
            // AttackGroup
            // 
            this.AttackGroup.Controls.Add(this.AttackList);
            resources.ApplyResources(this.AttackGroup, "AttackGroup");
            this.AttackGroup.Name = "AttackGroup";
            this.AttackGroup.TabStop = false;
            // 
            // AttackList
            // 
            this.AttackList.AllowDrop = true;
            this.AttackList.AllowUserToResizeRows = false;
            this.AttackList.BackgroundColor = System.Drawing.Color.PeachPuff;
            this.AttackList.ColumnHeadersHeightSizeMode = System.Windows.Forms.DataGridViewColumnHeadersHeightSizeMode.AutoSize;
            this.AttackList.Columns.AddRange(new System.Windows.Forms.DataGridViewColumn[] {
            this.NameCollum,
            this.ATCollum,
            this.TPCollum,
            this.KommentarCollum});
            resources.ApplyResources(this.AttackList, "AttackList");
            this.AttackList.Name = "AttackList";
            // 
            // NameCollum
            // 
            resources.ApplyResources(this.NameCollum, "NameCollum");
            this.NameCollum.Name = "NameCollum";
            // 
            // ATCollum
            // 
            this.ATCollum.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.ColumnHeader;
            resources.ApplyResources(this.ATCollum, "ATCollum");
            this.ATCollum.Name = "ATCollum";
            // 
            // TPCollum
            // 
            this.TPCollum.AutoSizeMode = System.Windows.Forms.DataGridViewAutoSizeColumnMode.ColumnHeader;
            resources.ApplyResources(this.TPCollum, "TPCollum");
            this.TPCollum.Name = "TPCollum";
            // 
            // KommentarCollum
            // 
            resources.ApplyResources(this.KommentarCollum, "KommentarCollum");
            this.KommentarCollum.Name = "KommentarCollum";
            // 
            // MeisterkommentarEdit
            // 
            resources.ApplyResources(this.MeisterkommentarEdit, "MeisterkommentarEdit");
            this.MeisterkommentarEdit.Name = "MeisterkommentarEdit";
            // 
            // MeisterkommentarLabel
            // 
            resources.ApplyResources(this.MeisterkommentarLabel, "MeisterkommentarLabel");
            this.MeisterkommentarLabel.Name = "MeisterkommentarLabel";
            // 
            // SaveButton
            // 
            resources.ApplyResources(this.SaveButton, "SaveButton");
            this.SaveButton.Name = "SaveButton";
            this.SaveButton.UseVisualStyleBackColor = true;
            this.SaveButton.Click += new System.EventHandler(this.SaveButton_Click);
            // 
            // LoadButton
            // 
            resources.ApplyResources(this.LoadButton, "LoadButton");
            this.LoadButton.Name = "LoadButton";
            this.LoadButton.UseVisualStyleBackColor = true;
            this.LoadButton.Click += new System.EventHandler(this.LoadButton_Click);
            // 
            // CritCreateForm
            // 
            this.AllowDrop = true;
            resources.ApplyResources(this, "$this");
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.SandyBrown;
            this.Controls.Add(this.LoadButton);
            this.Controls.Add(this.SaveButton);
            this.Controls.Add(this.MeisterkommentarLabel);
            this.Controls.Add(this.MeisterkommentarEdit);
            this.Controls.Add(this.AttackGroup);
            this.Controls.Add(this.LeLabel);
            this.Controls.Add(this.LeEdit);
            this.Controls.Add(this.SecondGroup);
            this.Controls.Add(this.VerteidugungGroup);
            this.Controls.Add(this.GrundwerteGroup);
            this.Controls.Add(this.NameEdit);
            this.Controls.Add(this.NameLabel);
            this.MaximizeBox = false;
            this.Name = "CritCreateForm";
            this.ShowIcon = false;
            this.SizeGripStyle = System.Windows.Forms.SizeGripStyle.Hide;
            this.DragDrop += new System.Windows.Forms.DragEventHandler(this.CritCreateForm_DragDrop);
            ((System.ComponentModel.ISupportInitialize)(this.LeEdit)).EndInit();
            this.GrundwerteGroup.ResumeLayout(false);
            this.GrundwerteGroup.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.KoEdit)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.AeEdit)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.AuEdit)).EndInit();
            this.VerteidugungGroup.ResumeLayout(false);
            this.VerteidugungGroup.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.PaEdit)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.MREdit)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.RSEdit)).EndInit();
            this.SecondGroup.ResumeLayout(false);
            this.SecondGroup.PerformLayout();
            ((System.ComponentModel.ISupportInitialize)(this.GWEdit)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.GsEdit)).EndInit();
            this.AttackGroup.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.AttackList)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label NameLabel;
        private System.Windows.Forms.TextBox NameEdit;
        private System.Windows.Forms.Label LeLabel;
        private System.Windows.Forms.NumericUpDown LeEdit;
        private System.Windows.Forms.GroupBox GrundwerteGroup;
        private System.Windows.Forms.Label AeLabel;
        private System.Windows.Forms.NumericUpDown AeEdit;
        private System.Windows.Forms.Label AuLabel;
        private System.Windows.Forms.NumericUpDown AuEdit;
        private System.Windows.Forms.GroupBox VerteidugungGroup;
        private System.Windows.Forms.Label MRLabel;
        private System.Windows.Forms.NumericUpDown MREdit;
        private System.Windows.Forms.Label RSLAbel;
        private System.Windows.Forms.NumericUpDown RSEdit;
        private System.Windows.Forms.Label KoLabel;
        private System.Windows.Forms.NumericUpDown KoEdit;
        private System.Windows.Forms.GroupBox SecondGroup;
        private System.Windows.Forms.Label GWLabel;
        private System.Windows.Forms.NumericUpDown GWEdit;
        private System.Windows.Forms.Label INILabel;
        private System.Windows.Forms.Label GsLabel;
        private System.Windows.Forms.NumericUpDown GsEdit;
        private System.Windows.Forms.TextBox INIEdit;
        private System.Windows.Forms.GroupBox AttackGroup;
        private System.Windows.Forms.DataGridView AttackList;
        private System.Windows.Forms.DataGridViewTextBoxColumn NameCollum;
        private System.Windows.Forms.DataGridViewTextBoxColumn ATCollum;
        private System.Windows.Forms.DataGridViewTextBoxColumn TPCollum;
        private System.Windows.Forms.DataGridViewTextBoxColumn KommentarCollum;
        private System.Windows.Forms.TextBox MeisterkommentarEdit;
        private System.Windows.Forms.Label MeisterkommentarLabel;
        private System.Windows.Forms.Button SaveButton;
        private System.Windows.Forms.Button LoadButton;
        private System.Windows.Forms.Label PaLabel;
        private System.Windows.Forms.NumericUpDown PaEdit;
    }
}

