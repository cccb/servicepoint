using ServicePoint;
using CompressionCode = ServicePoint.BindGen.CompressionCode;

using var connection = Connection.Open("127.0.0.1:2342");

connection.Send(Command.Clear().IntoPacket());
connection.Send(Command.Brightness(128).IntoPacket());

using var pixels = Bitmap.New(Constants.PixelWidth, Constants.PixelHeight);

for (var offset = 0; offset < int.MaxValue; offset++)
{
    pixels.Fill(false);

    for (var y = 0; y < pixels.Height; y++)
        pixels[(y + offset) % Constants.PixelWidth, y] = true;

    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone(), CompressionCode.Lzma).IntoPacket());
    Thread.Sleep(14);
}
