using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BitVec : SpNativeInstance<BindGen.BitVec>
{
    public static BitVec New(nuint size)
    {
        unsafe
        {
            return new BitVec(BitVecNative.sp_bitvec_new(size));
        }
    }

    public static BitVec Load(Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new BitVec(BitVecNative.sp_bitvec_load(bytesPtr, (nuint)bytes.Length));
            }
        }
    }

    public BitVec Clone()
    {
        unsafe
        {
            return new BitVec(Instance->Clone());
        }
    }

    public bool this[nuint index]
    {
        get
        {
            unsafe
            {
                return Instance->Get(index);
            }
        }
        set
        {
            unsafe
            {
                Instance->Set(index, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            Instance->Fill(value);
        }
    }

    public nuint Length
    {
        get
        {
            unsafe
            {
                return Instance->Len();
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                return Instance->UnsafeDataRef().AsSpan();
            }
        }
    }

    private unsafe BitVec(BindGen.BitVec* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => Instance->Free();
}
