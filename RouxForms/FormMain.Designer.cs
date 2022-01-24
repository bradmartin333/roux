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
            this.BtnTestWindow = new System.Windows.Forms.Button();
            this.RdTower = new System.Windows.Forms.RadioButton();
            this.RdTall = new System.Windows.Forms.RadioButton();
            this.SuspendLayout();
            // 
            // RdCar
            // 
            this.RdCar.AutoSize = true;
            this.RdCar.Checked = true;
            this.RdCar.Location = new System.Drawing.Point(35, 12);
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
            this.RdCube.Location = new System.Drawing.Point(35, 38);
            this.RdCube.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdCube.Name = "RdCube";
            this.RdCube.Size = new System.Drawing.Size(53, 19);
            this.RdCube.TabIndex = 1;
            this.RdCube.Text = "Cube";
            this.RdCube.UseVisualStyleBackColor = true;
            // 
            // BtnTestWindow
            // 
            this.BtnTestWindow.Location = new System.Drawing.Point(14, 115);
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
            this.RdTower.Location = new System.Drawing.Point(35, 63);
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
            this.RdTall.Location = new System.Drawing.Point(35, 88);
            this.RdTall.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.RdTall.Name = "RdTall";
            this.RdTall.Size = new System.Drawing.Size(42, 19);
            this.RdTall.TabIndex = 6;
            this.RdTall.Text = "Tall";
            this.RdTall.UseVisualStyleBackColor = true;
            // 
            // FormMain
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(122, 154);
            this.Controls.Add(this.RdTall);
            this.Controls.Add(this.RdTower);
            this.Controls.Add(this.BtnTestWindow);
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
        private Button BtnTestWindow;
        private RadioButton RdTower;
        private RadioButton RdTall;
    }
}

