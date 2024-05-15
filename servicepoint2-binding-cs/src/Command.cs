using System.Diagnostics.CodeAnalysis;
using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class Command : Sp2NativeInstance<BindGen.Command>
{
    public static bool TryFromPacket(Packet packet, [MaybeNullWhen(false)] out Command command)
    {
        unsafe
        {
            var result = NativeMethods.sp2_command_try_from_packet(packet.Into());
            if (result == null)
            {
                command = null;
                return false;
            }

            command = new Command(result);
            return true;
        }
    }

    public Command Clone()
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_clone(Instance));
        }
    }

    public static Command Clear()
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_clear());
        }
    }

    public static Command HardReset()
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_hard_reset());
        }
    }

    public static Command FadeOut()
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_fade_out());
        }
    }

    public static Command Brightness(byte brightness)
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_brightness(brightness));
        }
    }

    public static Command CharBrightness(int x, int y, ByteGrid grid)
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_char_brightness((ushort)x, (ushort)y, grid.Into()));
        }
    }

    public static Command BitmapLinear(int offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                NativeMethods.sp2_command_bitmap_linear((ushort)offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearAnd(int offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                NativeMethods.sp2_command_bitmap_linear_and((ushort)offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearOr(int offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                NativeMethods.sp2_command_bitmap_linear_or((ushort)offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearXor(int offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                NativeMethods.sp2_command_bitmap_linear_xor((ushort)offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearWin(int x, int y, PixelGrid pixelGrid)
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_bitmap_linear_win((ushort)x, (ushort)y, pixelGrid.Into()));
        }
    }

    public static Command Cp437Data(int x, int y, ByteGrid byteGrid)
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_cp437_data((ushort)x, (ushort)y, byteGrid.Into()));
        }
    }

    private unsafe Command(BindGen.Command* instance) : base(instance)
    {
    }

    private protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_command_dealloc(Instance);
    }
}
