using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class ByteGrid : Sp2NativeInstance<BindGen.ByteGrid>
{
    public static ByteGrid New(int width, int height)
    {
        unsafe
        {
            return new ByteGrid(NativeMethods.sp2_byte_grid_new((nuint)width, (nuint)height));
        }
    }

    public static ByteGrid Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new ByteGrid(NativeMethods.sp2_byte_grid_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public ByteGrid Clone()
    {
        unsafe
        {
            return new ByteGrid(NativeMethods.sp2_byte_grid_clone(Instance));
        }
    }

    public byte this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp2_byte_grid_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp2_byte_grid_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            NativeMethods.sp2_byte_grid_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp2_byte_grid_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp2_byte_grid_height(Instance);
            }
        }
    }

    private unsafe ByteGrid(BindGen.ByteGrid* instance) : base(instance)
    {
    }

    protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_byte_grid_dealloc(Instance);
    }
}
