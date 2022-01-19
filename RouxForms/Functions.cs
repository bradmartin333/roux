using System.Drawing.Imaging;
using System.Runtime.InteropServices;

namespace RouxForms
{
    public class Functions
    {
        [DllImport("roux.dll")]
        unsafe public static extern uint test_window(uint len, byte* ptr, int wid, int hgt, int x, int y);

        unsafe public static uint TestWindow(Bitmap bmp, double size, Point center)
        {
            // Predict size after red channel extraction
            Size bmpSize = bmp.Width % 12 == 0 ? bmp.Size : new Size(bmp.Width / 12 * 12, bmp.Height);

            // How much to shrink the image by to fit in a large window
            double scale = Math.Max(bmpSize.Width, bmpSize.Height) / size;

            // Clone bitmap and scale down
            bmp = new Bitmap((Bitmap)bmp.Clone(), new Size((int)(bmpSize.Width / scale), (int)(bmpSize.Height / scale)));

            // Extract red channel and resize to the predicted size
            byte[] data = GetRedChannelArr(ref bmp);

            // Send red channel data and image size info
            fixed (byte* p = &data[0])
                return test_window(
                    (uint)data.Length, p, 
                    bmp.Width, bmp.Height, 
                    center.X - bmp.Width / 2, center.Y - bmp.Height / 2);
        }

        [DllImport("roux.dll")]
        unsafe private static extern double get_entropy(uint len, byte* ptr);

        unsafe public static double GetEntropy(byte[] data)
        {
            fixed (byte* p = &data[0]) return get_entropy((uint)data.Length, p);
        }

        public static byte[] GetRedChannelArr(ref Bitmap bmp)
        {
            // Crop image down if it will create a border in BitmapData
            // Divisible by both 4 and 3 is the same as divisible by 12
            if (bmp.Width % 12 != 0)
                bmp = bmp.Clone(new Rectangle(Point.Empty, new Size(bmp.Width / 12 * 12, bmp.Height)), PixelFormat.Format24bppRgb);

            // Lock the bitmap's bits.  
            BitmapData bmpData = bmp.LockBits(new Rectangle(Point.Empty, bmp.Size), ImageLockMode.ReadWrite, PixelFormat.Format24bppRgb);

            // Get the address of the first line.
            IntPtr ptr = bmpData.Scan0;

            // Declare an array to hold the bytes of the bitmap.
            int bytes = Math.Abs(bmpData.Stride) * bmp.Height;
            byte[] rgbValues = new byte[bytes];
            byte[] rValues = new byte[bytes / 3];

            // Copy the RGB values into the array.
            Marshal.Copy(ptr, rgbValues, 0, bytes);

            // Copy every red channel value
            for (int counter = 2; counter < rgbValues.Length; counter += 3)
                rValues[counter / 3] = rgbValues[counter];

            return rValues;
        }
    }
}
