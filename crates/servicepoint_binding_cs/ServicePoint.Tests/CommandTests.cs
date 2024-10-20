namespace ServicePoint.Tests;

public class CommandTests
{
    private Connection _fakeConnection = Connection.Fake();

    [Fact]
    public void UseAfterSend()
    {
        var command = Command.Clear();
        _fakeConnection.Send(command);
        Assert.Throws<NullReferenceException>(() => _fakeConnection.Send(command));
        _fakeConnection.Send(Command.Clear());
    }
}
