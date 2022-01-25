using static RouxForms.Functions;

namespace RouxForms
{
    public partial class FormMain : Form
    {
        private readonly string CustomDir = @"C:\Users\delta\Desktop\pix\";
        private string[] CustomPics = Array.Empty<string>();
        private int CustomIter = 0;

        public FormMain()
        {
            InitializeComponent();
            if (Directory.Exists(CustomDir))
            {
                CustomPics = Directory.GetFiles(CustomDir);
                if (CustomPics.Length >= 2) BtnIterCustom.Enabled = true;
            }
        }

        private void BtnTestWindow_Click(object sender, EventArgs e)
        {
            Rectangle bounds = Screen.FromControl(this).Bounds;
            SizeF size = new((int)(bounds.Width * 0.9), (int)(bounds.Height * 0.9));
            ToggleRd(false);
            TestWindow(GetSelectedImage(), size, initialImage: true);
            ToggleRd(true);
        }

        private Bitmap GetSelectedImage()
        {
            if (RdCar.Checked)
                return Properties.Resources.car;
            else if (RdCube.Checked)
                return Properties.Resources.cube;
            else if (RdTower.Checked)
                return Properties.Resources.tower;
            else if (RdTall.Checked)
                return Properties.Resources.tall;
            else
                return Properties.Resources.chip;
        }

        private void ToggleRd(bool enable)
        {
            foreach (RadioButton rd in Controls.OfType<RadioButton>())
                rd.Enabled = enable;
        }

        private void BtnIterCustom_Click(object sender, EventArgs e)
        {
            CustomIter++;
            if (CustomIter > CustomPics.Length - 1) CustomIter = 2;
            Bitmap bmp = new(CustomPics[CustomIter]);
            Rectangle bounds = Screen.FromControl(this).Bounds;
            SizeF size = new((int)(bounds.Width * 0.9), (int)(bounds.Height * 0.9));
            if (CustomIter == 1)
            {
                ToggleRd(false);
                BtnTestWindow.Enabled = false;
                TestWindow(bmp, size, initialImage: true);
                ToggleRd(true);
                BtnTestWindow.Enabled = true;
            }
            else
                TestWindow(bmp, size);
        }
    }
}
