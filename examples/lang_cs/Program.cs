using ServicePoint2;

using var connection = Connection.Open("127.0.0.1:2342");

connection.Send(Command.Clear().IntoPacket());
connection.Send(Command.Brightness(128).IntoPacket());

using var pixels = PixelGrid.New(Constants.PixelWidth, Constants.PixelHeight);

for (var offset = 0; offset < int.MaxValue; offset++)
{
    pixels.Fill(false);

    for (var y = 0; y < pixels.Height; y++)
        pixels[(y + offset) % Constants.PixelWidth, y] = true;

    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone()).IntoPacket());
    Thread.Sleep(14);
}
