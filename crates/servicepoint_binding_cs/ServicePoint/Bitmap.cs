namespace ServicePoint;

public sealed partial class Bitmap
{
    public bool this[nuint x, nuint y]
    {
        get => Get(x, y);
        set => Set(x, y, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
