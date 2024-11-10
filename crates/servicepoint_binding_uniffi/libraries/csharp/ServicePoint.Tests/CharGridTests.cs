namespace ServicePoint.Tests;

public class CharGridTests
{
    [Fact]
    public void BasicFunctions()
    {
        var grid = new CharGrid(8, 2);
        Assert.Equal("\0", grid.Get(0, 0));
        Assert.Equal("\0", grid.Get(grid.Width() - 1, grid.Height() - 1));
        grid.Fill(" ");
        Assert.Equal(" ", grid.Get(1, 1));
        grid.Set(1, 1, "-");
        Assert.Equal("-", grid.Get(1, 1));
        Assert.Throws<PanicException>(() => grid.Get(8, 2));
    }

    [Fact]
    public void RowAndCol()
    {
        var grid = new CharGrid(3, 2);
        Assert.Equal("\0\0\0", grid.GetRow(0));
        grid.Fill(" ");
        Assert.Equal("  ", grid.GetCol(1));
        Assert.Throws<CharGridException.OutOfBounds>(() => grid.GetCol(3));
        Assert.Throws<CharGridException.InvalidSeriesLength>(() => grid.SetRow(1, "Text"));
        grid.SetRow(1, "Foo");
        Assert.Equal("Foo", grid.GetRow(1));
        Assert.Equal(" o", grid.GetCol(2));
    }
}
