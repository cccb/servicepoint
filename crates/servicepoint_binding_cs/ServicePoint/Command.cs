using System.Diagnostics.CodeAnalysis;

namespace ServicePoint;

public sealed partial class Command
{
    public static bool TryFromPacket(Packet packet, [MaybeNullWhen(false)] out Command command)
    {
        return (command = TryFromPacket(packet)) != null;
    }
}
