using System.Diagnostics.CodeAnalysis;

namespace ServicePoint2;

public static class ServicePoint2Extensions
{
    public static Packet IntoPacket(this Command command)
    {
        return Packet.FromCommand(command);
    }

    public static bool TryIntoCommand(this Packet packet, [MaybeNullWhen(false)] out Command command)
    {
        return Command.TryFromPacket(packet, out command);
    }
}
