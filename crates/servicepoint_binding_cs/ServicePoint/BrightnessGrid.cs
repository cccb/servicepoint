namespace ServicePoint;

public sealed partial class BrightnessGrid
{
    public static BrightnessGrid Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return Load(width, height, bytesPtr, (nuint)bytes.Length);
            }
        }
    }

    public byte this[nuint x, nuint y]
    {
        get => Get(x, y);
        set => Set(x, y, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
