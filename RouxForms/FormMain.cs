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
            if (radioButton1.Checked)
            {
                int clicks = test_window(0, 255);
                System.Diagnostics.Debug.WriteLine(clicks);
            }
        }
    }
}
