using ServicePoint;

using var connection = Connection.Open("127.0.0.1:2342");
if (connection == null)
{
    Console.Error.WriteLine("could not connect");
    return;
}

connection.Send(Command.Clear().IntoPacket());
connection.Send(Command.Brightness(128).IntoPacket());

using var pixels = new Bitmap(Constants.PixelWidth, Constants.PixelHeight);

for (nuint offset = 0; offset < nuint.MaxValue; offset++)
{
    pixels.Fill(false);

    for (nuint y = 0; y < pixels.Height(); y++)
        pixels[(y + offset) % Constants.PixelWidth, y] = true;

    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone(), CompressionCode.Lzma));
    Thread.Sleep(14);
}
