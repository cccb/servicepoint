using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class PixelGrid : Sp2NativeInstance<BindGen.PixelGrid>
{
    public static PixelGrid New(int width, int height)
    {
        unsafe
        {
            return new PixelGrid(NativeMethods.sp2_pixel_grid_new((nuint)width, (nuint)height));
        }
    }

    public static PixelGrid Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new PixelGrid(NativeMethods.sp2_pixel_grid_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public PixelGrid Clone()
    {
        unsafe
        {
            return new PixelGrid(NativeMethods.sp2_pixel_grid_clone(Instance));
        }
    }

    public bool this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp2_pixel_grid_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp2_pixel_grid_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            NativeMethods.sp2_pixel_grid_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp2_pixel_grid_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp2_pixel_grid_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var ptr = NativeMethods.sp2_pixel_grid_data_ref(Instance);
                return new Span<byte>(ptr, Width * Height / 8);
            }
        }
    }

    private unsafe PixelGrid(BindGen.PixelGrid* instance) : base(instance)
    {
    }

    protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_pixel_grid_dealloc(Instance);
    }
}
