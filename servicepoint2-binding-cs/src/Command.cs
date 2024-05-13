using System.Diagnostics.CodeAnalysis;
using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class Command : Sp2NativeInstance<BindGen.Command>
{
    public Command Clone()
    {
        unsafe
        {
            return new Command(NativeMethods.sp2_command_clone(Instance));
        }
    }

    public static bool TryLoad(Span<byte> bytes, [MaybeNullWhen(false)] out Command command)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                var instance = NativeMethods.sp2_command_try_load(bytesPtr, (nuint)bytes.Length);
                command = instance == null
                    ? null
                    : new Command(instance);
                return command != null;
            }
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

    protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_command_dealloc(Instance);
    }
}
