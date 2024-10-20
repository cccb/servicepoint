namespace ServicePoint.Tests;

public class BitmapTests
{
    [Fact]
    public void UseAfterFree()
    {
        var bitmap = Bitmap.NewScreenSized();
        _ = Command.BitmapLinearWin(0, 0, bitmap, CompressionCode.Uncompressed);
        Assert.Throws<NullReferenceException>(() => _ = Command.BitmapLinearWin(0, 0, bitmap, CompressionCode.Uncompressed));
    }
}
