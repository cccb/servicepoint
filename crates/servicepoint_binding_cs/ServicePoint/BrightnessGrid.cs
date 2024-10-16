using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BrightnessGrid : SpNativeInstance<SPBrightnessGrid>
{
    public static BrightnessGrid New(nuint width, nuint height)
    {
        unsafe
        {
            return new BrightnessGrid(BrightnessGridNative.sp_brightness_grid_new(width, height));
        }
    }

    public static BrightnessGrid Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new BrightnessGrid(BrightnessGridNative.sp_brightness_grid_load(width, height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public BrightnessGrid Clone()
    {
        unsafe
        {
            return new BrightnessGrid(BrightnessGridNative.sp_brightness_grid_clone(Instance));
        }
    }

    public byte this[nuint x, nuint y]
    {
        get
        {
            unsafe
            {
                return BrightnessGridNative.sp_brightness_grid_get(Instance, x, y);
            }
        }
        set
        {
            unsafe
            {
                BrightnessGridNative.sp_brightness_grid_set(Instance, x, y, value);
            }
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            BrightnessGridNative.sp_brightness_grid_fill(Instance, value);
        }
    }

    public nuint Width
    {
        get
        {
            unsafe
            {
                return BrightnessGridNative.sp_brightness_grid_width(Instance);
            }
        }
    }

    public nuint Height
    {
        get
        {
            unsafe
            {
                return BrightnessGridNative.sp_brightness_grid_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = BrightnessGridNative.sp_brightness_grid_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe BrightnessGrid(SPBrightnessGrid* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => BrightnessGridNative.sp_brightness_grid_free(Instance);
}
