using System.Text;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Cp437Grid : SpNativeInstance<BindGen.Cp437Grid>
{
    public static Cp437Grid New(int width, int height)
    {
        unsafe
        {
            return new Cp437Grid(NativeMethods.sp_cp437_grid_new((nuint)width, (nuint)height));
        }
    }

    public static Cp437Grid Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new Cp437Grid(NativeMethods.sp_cp437_grid_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public Cp437Grid Clone()
    {
        unsafe
        {
            return new Cp437Grid(NativeMethods.sp_cp437_grid_clone(Instance));
        }
    }

    public byte this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp_cp437_grid_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp_cp437_grid_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public string this[int y]
    {
        set
        {
            var width = Width;
            ArgumentOutOfRangeException.ThrowIfGreaterThan(value.Length, width);

            var x = 0;
            for (; x < value.Length; x++)
                this[x, y] = (byte)value[x];

            for (; x < width; x++)
                this[x, y] = 0;
        }

        get
        {
            var sb = new StringBuilder();
            for (int x = 0; x < Width; x++)
            {
                var val = this[x, y];
                if (val == 0)
                    break;
                sb.Append((char)val);
            }

            return sb.ToString();
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            NativeMethods.sp_cp437_grid_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_cp437_grid_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_cp437_grid_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp_cp437_grid_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe Cp437Grid(BindGen.Cp437Grid* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => NativeMethods.sp_cp437_grid_free(Instance);
}
