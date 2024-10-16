using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Bitmap : SpNativeInstance<BindGen.Bitmap>
{
    public static Bitmap New(nuint width, nuint height)
    {
        unsafe
        {
            return new Bitmap(BitmapNative.sp_bitmap_new(width, height));
        }
    }

    public static Bitmap Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new Bitmap(BitmapNative.sp_bitmap_load(width, height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public Bitmap Clone()
    {
        unsafe
        {
            return new Bitmap(Instance->Clone());
        }
    }

    public bool this[nuint x, nuint y]
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

    public void Fill(bool value)
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
                var slice = Instance->UnsafeDataRef();
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe Bitmap(BindGen.Bitmap* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => Instance->Free();
}
