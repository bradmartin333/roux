using static RouxForms.Functions;

namespace RouxForms
{
    public partial class FormMain : Form
    {
        private int TestEditPixels = 0;

        public FormMain()
        {
            InitializeComponent();
        }

        private void BtnGetEntropy_Click(object sender, EventArgs e)
        {
            Bitmap bmp = GetSelectedImage();
            LabelMain.Text = GetEntropy(GetRedChannelArr(ref bmp, 1)).ToString();
        }

        private void BtnTestWindow_Click(object sender, EventArgs e)
        {
            TestEditPixels++;
            Rectangle bounds = Screen.FromControl(this).Bounds;
            SizeF size = new((int)(bounds.Width * 0.9), (int)(bounds.Height * 0.9));
            if (TestEditPixels == 1)
            {
                ToggleRd(false);
                LabelMain.Text = $"{TestWindow(GetSelectedImage(), size, TestEditPixels)} clicks";
                ToggleRd(true);
            }   
            else
                TestWindow(GetSelectedImage(), size, TestEditPixels);
        }

        private Bitmap GetSelectedImage()
        {
            if (RdCar.Checked)
                return Properties.Resources.car;
            else if (RdCube.Checked)
                return Properties.Resources.cube;
            else if (RdTower.Checked)
                return Properties.Resources.tower;
            else
                return Properties.Resources.tall;
        }

        private void ToggleRd(bool enable)
        {
            foreach (RadioButton rd in Controls.OfType<RadioButton>())
            {
                rd.Enabled = enable;
            }
        }

        private void Rd_CheckedChanged(object sender, EventArgs e)
        {
            TestEditPixels = 0;
        }
    }
}
