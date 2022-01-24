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

        private void BtnTestWindow_Click(object sender, EventArgs e)
        {
            TestEditPixels++;
            Rectangle bounds = Screen.FromControl(this).Bounds;
            SizeF size = new((int)(bounds.Width * 0.9), (int)(bounds.Height * 0.9));
            if (TestEditPixels == 1)
            {
                ToggleRd(false);
                TestWindow(GetSelectedImage(), size, initialImage: true);
                ToggleRd(true);
                TestEditPixels = 0;
            }   
            else
                TestWindow(GetSelectedImage(), size);
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
    }
}
