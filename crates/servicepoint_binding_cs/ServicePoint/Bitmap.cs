using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Bitmap : SpNativeInstance<SPBitmap>
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
            return new Bitmap(BitmapNative.sp_bitmap_clone(Instance));
        }
    }

    public bool this[nuint x, nuint y]
    {
        get
        {
            unsafe
            {
                return BitmapNative.sp_bitmap_get(Instance, x, y);
            }
        }
        set
        {
            unsafe
            {
                BitmapNative.sp_bitmap_set(Instance, x, y, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            BitmapNative.sp_bitmap_fill(Instance, value);
        }
    }

    public nuint Width
    {
        get
        {
            unsafe
            {
                return BitmapNative.sp_bitmap_width(Instance);
            }
        }
    }

    public nuint Height
    {
        get
        {
            unsafe
            {
                return BitmapNative.sp_bitmap_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = BitmapNative.sp_bitmap_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe Bitmap(SPBitmap* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => BitmapNative.sp_bitmap_free(Instance);
}
