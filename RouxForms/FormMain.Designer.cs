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
            this.BtnTestWindow = new System.Windows.Forms.Button();
            this.BtnIterCustom = new System.Windows.Forms.Button();
            this.SuspendLayout();
            // 
            // BtnTestWindow
            // 
            this.BtnTestWindow.Location = new System.Drawing.Point(32, 28);
            this.BtnTestWindow.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.BtnTestWindow.Name = "BtnTestWindow";
            this.BtnTestWindow.Size = new System.Drawing.Size(88, 27);
            this.BtnTestWindow.TabIndex = 4;
            this.BtnTestWindow.Text = "Test Window";
            this.BtnTestWindow.UseVisualStyleBackColor = true;
            this.BtnTestWindow.Click += new System.EventHandler(this.BtnTestWindow_Click);
            // 
            // BtnIterCustom
            // 
            this.BtnIterCustom.Enabled = false;
            this.BtnIterCustom.Location = new System.Drawing.Point(169, 28);
            this.BtnIterCustom.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.BtnIterCustom.Name = "BtnIterCustom";
            this.BtnIterCustom.Size = new System.Drawing.Size(88, 27);
            this.BtnIterCustom.TabIndex = 8;
            this.BtnIterCustom.Text = "Iter Custom";
            this.BtnIterCustom.UseVisualStyleBackColor = true;
            this.BtnIterCustom.Click += new System.EventHandler(this.BtnIterCustom_Click);
            // 
            // FormMain
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(284, 84);
            this.Controls.Add(this.BtnIterCustom);
            this.Controls.Add(this.BtnTestWindow);
            this.Margin = new System.Windows.Forms.Padding(4, 3, 4, 3);
            this.Name = "FormMain";
            this.Text = "Form1";
            this.ResumeLayout(false);

        }

        #endregion
        private Button BtnTestWindow;
        private Button BtnIterCustom;
    }
}

