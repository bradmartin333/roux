using System.Drawing.Imaging;
using System.Runtime.InteropServices;

namespace RouxForms
{
    public class Functions
    {
        private static readonly string TestPath = @"C:\Users\delta\Desktop\out.txt";

        unsafe private static byte* DataPointer;

        [DllImport("roux.dll")]
        unsafe private static extern void test_window(uint len, byte* ptr, uint wid, uint hgt, uint x, uint y, 
            [MarshalAs(UnmanagedType.LPUTF8Str)] string utf8Text);

        unsafe public static void TestWindow(Bitmap bmp, SizeF size, bool initialImage = false)
        {
            // Predict size after red channel extraction
            Size bmpSize = bmp.Width % 12 == 0 ? bmp.Size : new Size(bmp.Width / 12 * 12, bmp.Height);

            // Resize bitmap to fit a single screen
            bmp = new Bitmap((Bitmap)bmp.Clone(), new Size((int)(bmpSize.Width / (bmpSize.Height / size.Height)), (int)size.Height));

            // Extract red channel and resize to the predicted size
            byte[] data = GetRedChannelArr(ref bmp);

            if (initialImage) // Send red channel data and image size info to window
            {
                fixed (byte* p = &data[0])
                {
                    DataPointer = p;
                    test_window((uint)data.Length, p, (uint)bmp.Width, (uint)bmp.Height, 5, 5, TestPath);
                }
            }
            else // Update data
                for (int i = 0; i < data.Length; i++)
                    DataPointer[i] = data[i];
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
