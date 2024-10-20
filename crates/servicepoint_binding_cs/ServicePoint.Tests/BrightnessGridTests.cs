namespace ServicePoint.Tests;

public class BBrightnessGridTests
{
    [Fact]
    public void UseAfterFree()
    {
        var grid = new BrightnessGrid(23, 42);
        _ = Command.CharBrightness(0, 0, grid);
        Assert.Throws<NullReferenceException>(() => _ = Command.CharBrightness(0, 0, grid));
    }
}
