using static RouxForms.Functions;

namespace RouxForms
{
    public partial class FormMain : Form
    {
        public FormMain()
        {
            InitializeComponent();
        }

        private void Button1_Click(object sender, EventArgs e)
        {
            Bitmap bmp = GetSelectedImage();
            label1.Text = GetEntropy(GetRedChannelArr(ref bmp)).ToString();
        }

        private void Button2_Click(object sender, EventArgs e)
        {
            Rectangle bounds = Screen.FromControl(this).Bounds;
            SizeF size = new((int)(bounds.Width * 0.95), (int)(bounds.Height * 0.95));
            label1.Text = $"{TestWindow(GetSelectedImage(), size)} clicks";
        }

        private Bitmap GetSelectedImage()
        {
            if (radioButton1.Checked)
                return Properties.Resources.car;
            else if (radioButton2.Checked)
                return Properties.Resources.cube;
            else if (radioButton3.Checked)
                return Properties.Resources.tower;
            else
                return Properties.Resources.tall;
        }
    }
}
