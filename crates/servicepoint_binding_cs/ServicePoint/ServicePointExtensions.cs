using System.Diagnostics.CodeAnalysis;

namespace ServicePoint;

public static class ServicePointExtensions
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
