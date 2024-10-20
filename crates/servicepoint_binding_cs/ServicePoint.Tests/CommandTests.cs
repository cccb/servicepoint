namespace ServicePoint.Tests;

public class CommandTests
{
    private Connection _fakeConnection = Connection.Fake();

    [Fact]
    public void Test1()
    {
        var command = Command.Clear();
        _fakeConnection.Send(command);
        Assert.Throws<NullReferenceException>(() => _fakeConnection.Send(command));
    }
}
