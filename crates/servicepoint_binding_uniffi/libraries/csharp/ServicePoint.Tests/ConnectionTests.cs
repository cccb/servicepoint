namespace ServicePoint.Tests;

public class ConnectionTests
{
    [Fact]
    public void InvalidHostnameThrows()
    {
        Assert.Throws<ServicePointException.IoException>(() => new Connection(""));
        Assert.Throws<ServicePointException.IoException>(() => new Connection("-%6$§"));
    }
}
