namespace ServicePoint.Tests;

public class BitmapTests
{
    [Fact]
    public void BasicFunctions()
    {
        var bitmap = new Bitmap(8, 2);
        Assert.False(bitmap.Get(0, 0));
        Assert.False(bitmap.Get(bitmap.Width() - 1, bitmap.Height() - 1));
        bitmap.Fill(true);
        Assert.True(bitmap.Get(1, 1));
        bitmap.Set(1, 1, false);
        Assert.False(bitmap.Get(1, 1));
    }
}
