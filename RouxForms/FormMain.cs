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
            Bitmap bmp = radioButton1.Checked ? Properties.Resources.car : Properties.Resources.cube;
            label1.Text = GetEntropy(GetRedChannelArr(ref bmp)).ToString();
        }

        private void button2_Click(object sender, EventArgs e)
        {
            Bitmap bmp = radioButton1.Checked ? Properties.Resources.car : Properties.Resources.cube;

            Rectangle bounds = Screen.FromControl(this).Bounds;
            double size = Math.Min(bounds.Width, bounds.Height) * 0.95;
            Point center = new Point((int)(bounds.Width * 0.475), (int)(bounds.Height * 0.475));

            label1.Text = $"{TestWindow(bmp, size, center)} clicks";
        }
    }
}
