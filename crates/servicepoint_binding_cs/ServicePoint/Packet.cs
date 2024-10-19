using System.Diagnostics.CodeAnalysis;

namespace ServicePoint;

public sealed partial class Packet
{
    public static bool TryLoad(Span<byte> bytes, [MaybeNullWhen(false)] out Packet packet)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                packet = TryLoad(bytesPtr, (nuint)bytes.Length);
                return packet != null;
            }
        }
    }

    public bool TryIntoCommand([MaybeNullWhen(false)] out Command command)
    {
        return Command.TryFromPacket(this, out command);
    }
}
