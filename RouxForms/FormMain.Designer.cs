namespace RouxForms
{
    partial class FormMain
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.RdCar = new System.Windows.Forms.RadioButton();
            this.RdCube = new System.Windows.Forms.RadioButton();
            this.BtnGetEntropy = new System.Windows.Forms.Button();
            this.LabelMain = new System.Windows.Forms.Label();
            this.BtnTestWindow = new System.Windows.Forms.Button();
            this.RdTower = new System.Windows.Forms.RadioButton();
            this.RdTall = new System.Windows.Forms.RadioButton();
            this.BtnTestTiles = new System.Windows.Forms.Button();
            this.SuspendLayout();
            // 
            // RdCar
            // 
            this.RdCar.AutoSize = true;
            this.RdCar.Checked = true;
            this.RdCar.Location = new System.Drawing.Point(14, 14);
            this.RdCar.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdCar.Name = "RdCar";
            this.RdCar.Size = new System.Drawing.Size(43, 19);
            this.RdCar.TabIndex = 0;
            this.RdCar.TabStop = true;
            this.RdCar.Text = "Car";
            this.RdCar.UseVisualStyleBackColor = true;
            // 
            // RdCube
            // 
            this.RdCube.AutoSize = true;
            this.RdCube.Location = new System.Drawing.Point(14, 40);
            this.RdCube.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdCube.Name = "RdCube";
            this.RdCube.Size = new System.Drawing.Size(53, 19);
            this.RdCube.TabIndex = 1;
            this.RdCube.Text = "Cube";
            this.RdCube.UseVisualStyleBackColor = true;
            // 
            // BtnGetEntropy
            // 
            this.BtnGetEntropy.Location = new System.Drawing.Point(102, 14);
            this.BtnGetEntropy.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.BtnGetEntropy.Name = "BtnGetEntropy";
            this.BtnGetEntropy.Size = new System.Drawing.Size(88, 27);
            this.BtnGetEntropy.TabIndex = 2;
            this.BtnGetEntropy.Text = "Get Entropy";
            this.BtnGetEntropy.UseVisualStyleBackColor = true;
            this.BtnGetEntropy.Click += new System.EventHandler(this.BtnGetEntropy_Click);
            // 
            // LabelMain
            // 
            this.LabelMain.AutoSize = true;
            this.LabelMain.Location = new System.Drawing.Point(127, 45);
            this.LabelMain.Margin = new System.Windows.Forms.Padding(4, 0, 4, 0);
            this.LabelMain.Name = "LabelMain";
            this.LabelMain.Size = new System.Drawing.Size(29, 15);
            this.LabelMain.TabIndex = 3;
            this.LabelMain.Text = "N/A";
            // 
            // BtnTestWindow
            // 
            this.BtnTestWindow.Location = new System.Drawing.Point(198, 14);
            this.BtnTestWindow.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.BtnTestWindow.Name = "BtnTestWindow";
            this.BtnTestWindow.Size = new System.Drawing.Size(88, 27);
            this.BtnTestWindow.TabIndex = 4;
            this.BtnTestWindow.Text = "Test Window";
            this.BtnTestWindow.UseVisualStyleBackColor = true;
            this.BtnTestWindow.Click += new System.EventHandler(this.BtnTestWindow_Click);
            // 
            // RdTower
            // 
            this.RdTower.AutoSize = true;
            this.RdTower.Location = new System.Drawing.Point(14, 65);
            this.RdTower.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdTower.Name = "RdTower";
            this.RdTower.Size = new System.Drawing.Size(56, 19);
            this.RdTower.TabIndex = 5;
            this.RdTower.Text = "Tower";
            this.RdTower.UseVisualStyleBackColor = true;
            // 
            // RdTall
            // 
            this.RdTall.AutoSize = true;
            this.RdTall.Location = new System.Drawing.Point(14, 90);
            this.RdTall.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdTall.Name = "RdTall";
            this.RdTall.Size = new System.Drawing.Size(42, 19);
            this.RdTall.TabIndex = 6;
            this.RdTall.Text = "Tall";
            this.RdTall.UseVisualStyleBackColor = true;
            // 
            // BtnTestTiles
            // 
            this.BtnTestTiles.Location = new System.Drawing.Point(294, 14);
            this.BtnTestTiles.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.BtnTestTiles.Name = "BtnTestTiles";
            this.BtnTestTiles.Size = new System.Drawing.Size(88, 27);
            this.BtnTestTiles.TabIndex = 7;
            this.BtnTestTiles.Text = "Test Tiles";
            this.BtnTestTiles.UseVisualStyleBackColor = true;
            this.BtnTestTiles.Click += new System.EventHandler(this.BtnTestTiles_Click);
            // 
            // FormMain
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(387, 125);
            this.Controls.Add(this.BtnTestTiles);
            this.Controls.Add(this.RdTall);
            this.Controls.Add(this.RdTower);
            this.Controls.Add(this.BtnTestWindow);
            this.Controls.Add(this.LabelMain);
            this.Controls.Add(this.BtnGetEntropy);
            this.Controls.Add(this.RdCube);
            this.Controls.Add(this.RdCar);
            this.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.Name = "FormMain";
            this.Text = "Form1";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.RadioButton RdCar;
        private System.Windows.Forms.RadioButton RdCube;
        private System.Windows.Forms.Button BtnGetEntropy;
        private System.Windows.Forms.Label LabelMain;
        private Button BtnTestWindow;
        private RadioButton RdTower;
        private RadioButton RdTall;
        private Button BtnTestTiles;
    }
}

