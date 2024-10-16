using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BrightnessGrid : SpNativeInstance<BindGen.BrightnessGrid>
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
            return new BrightnessGrid(Instance->Clone());
        }
    }

    public byte this[nuint x, nuint y]
    {
        get
        {
            unsafe
            {
                return Instance->Get(x, y);
            }
        }
        set
        {
            unsafe
            {
                Instance->Set(x, y, value);
            }
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            Instance->Fill(value);
        }
    }

    public nuint Width
    {
        get
        {
            unsafe
            {
                return Instance->Width();
            }
        }
    }

    public nuint Height
    {
        get
        {
            unsafe
            {
                return Instance->Height();
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                return Instance->UnsafeDataRef().AsSpan();
            }
        }
    }

    private unsafe BrightnessGrid(BindGen.BrightnessGrid* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => Instance->Free();
}
