namespace ServicePoint.Tests;

public class BitVecTests
{
    [Fact]
    public void UseAfterFree()
    {
        var bitvec = new BitVec(8);
        _ = Command.BitmapLinear(0, bitvec, CompressionCode.Uncompressed);
        Assert.Throws<NullReferenceException>(() => _ = Command.BitmapLinear(0, bitvec, CompressionCode.Uncompressed));
    }
}
