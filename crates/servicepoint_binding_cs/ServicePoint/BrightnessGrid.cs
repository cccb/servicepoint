namespace ServicePoint;

public sealed partial class BrightnessGrid
{
    public byte this[nuint x, nuint y]
    {
        get => Get(x, y);
        set => Set(x, y, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
