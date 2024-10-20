using System.Runtime.CompilerServices;

namespace ServicePoint.Tests;

public class Cp437GridTests
{
    [Fact]
    public void UseAfterFree()
    {
        var grid = new Cp437Grid(2, 3);
        _ = Command.Cp437Data(0, 0, grid.Clone());
        _ = Command.Cp437Data(0, 0, grid);
        Assert.Throws<NullReferenceException>(() => _ = Command.Cp437Data(0, 0, grid));
    }

    [Fact]
    public void ReadAndWriteString()
    {
        var grid = new Cp437Grid(3, 2);
        grid[1] = "abc";
        Assert.Equal("abc", grid[1]);
    }

    [Fact]
    public void LoadSpan()
    {
        var ascii_str = "abc123"u8;
        var grid = Cp437Grid.Load(3, 2, ascii_str);
        Assert.Equal("abc", grid[0]);
        Assert.Equal("123", grid[1]);
    }
}
