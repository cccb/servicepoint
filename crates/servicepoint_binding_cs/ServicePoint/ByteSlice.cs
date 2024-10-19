namespace ServicePoint;

public partial struct SPByteSlice
{
    public unsafe Span<byte> AsSpan() => new (start, (int)length);
}
