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
            Bitmap bmp = new(Properties.Resources.car);
            if (radioButton2.Checked) bmp = new Bitmap(Properties.Resources.cube);
            Bitmap resized = new(bmp, new Size(bmp.Width / 5, bmp.Height / 5));
            label1.Text = GetEntropy(GetRedChannelArr(ref resized)).ToString();
        }
    }
}
