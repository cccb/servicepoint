using System.Threading;
using ServicePoint;

var connection = new Connection("127.0.0.1:2342");
connection.Send(Command.Clear());

connection.Send(Command.Brightness(5));

var pixels = Bitmap.NewMaxSized();
for (ulong offset = 0; offset < ulong.MaxValue; offset++)
{
    pixels.Fill(false);

    for (ulong y = 0; y < pixels.Height(); y++)
        pixels.Set((y + offset) % pixels.Width(), y, true);

    connection.Send(Command.BitmapLinearWin(0, 0, pixels));
    Thread.Sleep(14);
}
