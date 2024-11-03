namespace ServicePoint.Tests;

public class CommandTests
{
    private Connection _connection = Connection.NewFake();

    [Fact]
    public void ClearSendable()
    {
        _connection.Send(Command.Clear());
    }

    [Fact]
    public void BrightnessSendable()
    {
        _connection.Send(Command.Brightness(5));
    }

    [Fact]
    public void InvalidBrightnessThrows()
    {
        Assert.Throws<ServicePointException.InvalidBrightness>(() => Command.Brightness(42));
    }

    [Fact]
    public void FadeOutSendable()
    {
        _connection.Send(Command.FadeOut());
    }

    [Fact]
    public void HardResetSendable()
    {
        _connection.Send(Command.HardReset());
    }
}
