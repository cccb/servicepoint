using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BitVec : SpNativeInstance<SPBitVec>
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
            return new BitVec(BitVecNative.sp_bitvec_clone(Instance));
        }
    }

    public bool this[nuint index]
    {
        get
        {
            unsafe
            {
                return BitVecNative.sp_bitvec_get(Instance, index);
            }
        }
        set
        {
            unsafe
            {
                BitVecNative.sp_bitvec_set(Instance, index, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            BitVecNative.sp_bitvec_fill(Instance, value);
        }
    }

    public nuint Length
    {
        get
        {
            unsafe
            {
                return BitVecNative.sp_bitvec_len(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = BitVecNative.sp_bitvec_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe BitVec(SPBitVec* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => BitVecNative.sp_bitvec_free(Instance);
}
