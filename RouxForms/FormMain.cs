using static RouxForms.Functions;

namespace RouxForms
{
    public partial class FormMain : Form
    {
        private readonly string Dir = @"C:\Users\delta\Desktop\pix\";
        private string[] Pics = Array.Empty<string>();
        private int Iter = 0;
        private SizeF WindowSize = SizeF.Empty;

        public FormMain()
        {
            InitializeComponent();
            if (Directory.Exists(Dir))
            {
                Pics = Directory.GetFiles(Dir);
                if (Pics.Length >= 2) BtnIterCustom.Enabled = true;
            }
            Rectangle bounds = Screen.FromControl(this).Bounds;
            WindowSize = new((int)(bounds.Width * 0.9), (int)(bounds.Height * 0.9));
        }
        private void BtnTestWindow_Click(object sender, EventArgs e)
        {
            BtnIterCustom.Enabled = false;
            BtnTestWindow.Enabled = false;
            TestWindow(Properties.Resources.cube, WindowSize, initialImage: true);
            BtnIterCustom.Enabled = true;
            BtnTestWindow.Enabled = true;
        }

        private void BtnIterCustom_Click(object sender, EventArgs e)
        {
            Iter++;
            if (Iter > Pics.Length - 1) Iter = 2;
            if (Iter == 1)
            {
                BtnTestWindow.Enabled = false;
                TestWindow(new(Pics[Iter]), WindowSize, initialImage: true);
                BtnTestWindow.Enabled = true;
                Iter = 0;
            }
            else
                TestWindow(new(Pics[Iter]), WindowSize);
        }
    }
}
