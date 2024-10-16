using System.Diagnostics.CodeAnalysis;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Command : SpNativeInstance<BindGen.Command>
{
    public static bool TryFromPacket(Packet packet, [MaybeNullWhen(false)] out Command command)
    {
        unsafe
        {
            var result = CommandNative.sp_command_try_from_packet(packet.Into());
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
            return new Command(Instance->Clone());
        }
    }

    public static Command Clear()
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_clear());
        }
    }

    public static Command HardReset()
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_hard_reset());
        }
    }

    public static Command FadeOut()
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_fade_out());
        }
    }

    public static Command Brightness(byte brightness)
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_brightness(brightness));
        }
    }

    public static Command CharBrightness(ushort x, ushort y, BrightnessGrid grid)
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_char_brightness(x, y, grid.Into()));
        }
    }

    public static Command BitmapLinear(ushort offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                CommandNative.sp_command_bitmap_linear(offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearAnd(ushort offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                CommandNative.sp_command_bitmap_linear_and(offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearOr(ushort offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                CommandNative.sp_command_bitmap_linear_or(offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearXor(ushort offset, BitVec bitVec, CompressionCode compressionCode)
    {
        unsafe
        {
            return new Command(
                CommandNative.sp_command_bitmap_linear_xor(offset, bitVec.Into(), compressionCode));
        }
    }

    public static Command BitmapLinearWin(ushort x, ushort y, Bitmap bitmap, CompressionCode compression)
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_bitmap_linear_win(x, y, bitmap.Into(), compression));
        }
    }

    public static Command Cp437Data(ushort x, ushort y, Cp437Grid byteGrid)
    {
        unsafe
        {
            return new Command(CommandNative.sp_command_cp437_data(x, y, byteGrid.Into()));
        }
    }

    private unsafe Command(BindGen.Command* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => Instance->Free();
}
