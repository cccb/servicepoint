using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class PixelGrid : SpNativeInstance<BindGen.PixelGrid>
{
    public static PixelGrid New(int width, int height)
    {
        unsafe
        {
            return new PixelGrid(NativeMethods.sp_pixel_grid_new((nuint)width, (nuint)height));
        }
    }

    public static PixelGrid Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new PixelGrid(NativeMethods.sp_pixel_grid_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public PixelGrid Clone()
    {
        unsafe
        {
            return new PixelGrid(NativeMethods.sp_pixel_grid_clone(Instance));
        }
    }

    public bool this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp_pixel_grid_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp_pixel_grid_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            NativeMethods.sp_pixel_grid_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_pixel_grid_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_pixel_grid_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp_pixel_grid_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe PixelGrid(BindGen.PixelGrid* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => NativeMethods.sp_pixel_grid_free(Instance);
}
