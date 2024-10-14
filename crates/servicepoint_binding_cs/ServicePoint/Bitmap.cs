using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Bitmap : SpNativeInstance<BindGen.Bitmap>
{
    public static Bitmap New(int width, int height)
    {
        unsafe
        {
            return new Bitmap(NativeMethods.sp_bitmap_new((nuint)width, (nuint)height));
        }
    }

    public static Bitmap Load(int width, int height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new Bitmap(NativeMethods.sp_bitmap_load((nuint)width, (nuint)height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public Bitmap Clone()
    {
        unsafe
        {
            return new Bitmap(NativeMethods.sp_bitmap_clone(Instance));
        }
    }

    public bool this[int x, int y]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp_bitmap_get(Instance, (nuint)x, (nuint)y);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp_bitmap_set(Instance, (nuint)x, (nuint)y, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            NativeMethods.sp_bitmap_fill(Instance, value);
        }
    }

    public int Width
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_bitmap_width(Instance);
            }
        }
    }

    public int Height
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_bitmap_height(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp_bitmap_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe Bitmap(BindGen.Bitmap* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => NativeMethods.sp_bitmap_free(Instance);
}
