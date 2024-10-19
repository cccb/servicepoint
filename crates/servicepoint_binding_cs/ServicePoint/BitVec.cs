namespace ServicePoint;

public sealed partial class BitVec
{
    public bool this[nuint index]
    {
        get => Get(index);
        set => Set(index, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
