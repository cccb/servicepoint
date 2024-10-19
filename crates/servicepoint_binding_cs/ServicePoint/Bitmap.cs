namespace ServicePoint;

public sealed partial class Bitmap
{
    public static Bitmap Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return Load(width, height, bytesPtr, (nuint)bytes.Length);
            }
        }
    }

    public bool this[nuint x, nuint y]
    {
        get => Get(x, y);
        set => Set(x, y, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
