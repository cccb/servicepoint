using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class BitVec : SpNativeInstance<BindGen.BitVec>
{
    public static BitVec New(int size)
    {
        unsafe
        {
            return new BitVec(NativeMethods.sp_bitvec_new((nuint)size));
        }
    }

    public static BitVec Load(Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new BitVec(NativeMethods.sp_bitvec_load(bytesPtr, (nuint)bytes.Length));
            }
        }
    }

    public BitVec Clone()
    {
        unsafe
        {
            return new BitVec(NativeMethods.sp_bitvec_clone(Instance));
        }
    }

    public bool this[int index]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp_bitvec_get(Instance, (nuint)index);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp_bitvec_set(Instance, (nuint)index, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            NativeMethods.sp_bitvec_fill(Instance, value);
        }
    }

    public int Length
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp_bitvec_len(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp_bitvec_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe BitVec(BindGen.BitVec* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => NativeMethods.sp_bitvec_free(Instance);
}
