using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BrightnessGrid : SpNativeInstance<BindGen.BrightnessGrid>
{
    public static BrightnessGrid New(int width, int height)
    {
        unsafe
        {
            return new BrightnessGrid(NativeMethods.sp_brightness_grid_new((nuint)width, (nuint)height));
        }
    }

    public static BrightnessGrid Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new BrightnessGrid(NativeMethods.sp_brightness_grid_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public BrightnessGrid Clone()
    {
        unsafe
        {
            return new BrightnessGrid(NativeMethods.sp_brightness_grid_clone(Instance));
        }
    }

    public byte this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp_brightness_grid_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp_brightness_grid_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            NativeMethods.sp_brightness_grid_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_brightness_grid_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_brightness_grid_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp_brightness_grid_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe BrightnessGrid(BindGen.BrightnessGrid* instance) : base(instance)
    {
    }

    private protected override unsafe void Dealloc()
    {
        NativeMethods.sp_brightness_grid_dealloc(Instance);
    }
}
