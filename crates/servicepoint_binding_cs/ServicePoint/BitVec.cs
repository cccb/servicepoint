namespace ServicePoint;

public sealed partial class BitVec
{
    public static BitVec Load(ReadOnlySpan<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return Load(bytesPtr, (nuint)bytes.Length);
            }
        }
    }

    public bool this[nuint index]
    {
        get => Get(index);
        set => Set(index, value);
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
