namespace ServicePoint;

public partial struct SPByteSlice
{
    public unsafe Span<byte> AsSpan() => new Span<byte>(start, (int)length);
}
