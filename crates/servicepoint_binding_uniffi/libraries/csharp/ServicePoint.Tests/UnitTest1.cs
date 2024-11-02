using ServicePoint;

namespace ServicePoint.Tests;

public class UnitTest1
{
    [Fact]
    public void Test1()
    {
        Assert.Throws<ConnectionException.IoException>(() => new Connection(""));
    }
}
