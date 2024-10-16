using System.Diagnostics.CodeAnalysis;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Packet : SpNativeInstance<SPPacket>
{
    public static Packet FromCommand(Command command)
    {
        unsafe
        {
            return new Packet(PacketNative.sp_packet_from_command(command.Into()));
        }
    }

    public static bool TryFromBytes(Span<byte> bytes, [MaybeNullWhen(false)] out Packet packet)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                var instance = PacketNative.sp_packet_try_load(bytesPtr, (nuint)bytes.Length);
                packet = instance == null
                    ? null
                    : new Packet(instance);
                return packet != null;
            }
        }
    }

    private unsafe Packet(SPPacket* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => PacketNative.sp_packet_free(Instance);
}
