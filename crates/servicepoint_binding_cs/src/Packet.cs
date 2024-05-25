using System.Diagnostics.CodeAnalysis;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Packet : SpNativeInstance<BindGen.Packet>
{
    public static Packet FromCommand(Command command)
    {
        unsafe
        {
            return new Packet(NativeMethods.sp2_packet_from_command(command.Into()));
        }
    }

    public static bool TryFromBytes(Span<byte> bytes, [MaybeNullWhen(false)] out Packet packet)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                var instance = NativeMethods.sp2_packet_try_load(bytesPtr, (nuint)bytes.Length);
                packet = instance == null
                    ? null
                    : new Packet(instance);
                return packet != null;
            }
        }
    }

    private unsafe Packet(BindGen.Packet* instance) : base(instance)
    {
    }

    private protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_packet_dealloc(Instance);
    }
}
