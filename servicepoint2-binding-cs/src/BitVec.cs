using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class BitVec : Sp2NativeInstance<BindGen.BitVec>
{
    public static BitVec New(int size)
    {
        unsafe
        {
            return new BitVec(NativeMethods.sp2_bit_vec_new((nuint)size));
        }
    }

    public static BitVec Load(Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new BitVec(NativeMethods.sp2_bit_vec_load(bytesPtr, (nuint)bytes.Length));
            }
        }
    }

    public BitVec Clone()
    {
        unsafe
        {
            return new BitVec(NativeMethods.sp2_bit_vec_clone(Instance));
        }
    }

    public bool this[int index]
    {
        get
        {
            unsafe
            {
                return NativeMethods.sp2_bit_vec_get(Instance, (nuint)index);
            }
        }
        set
        {
            unsafe
            {
                NativeMethods.sp2_bit_vec_set(Instance, (nuint)index, value);
            }
        }
    }

    public void Fill(bool value)
    {
        unsafe
        {
            NativeMethods.sp2_bit_vec_fill(Instance, value);
        }
    }

    public int Length
    {
        get
        {
            unsafe
            {
                return (int)NativeMethods.sp2_bit_vec_len(Instance);
            }
        }
    }

    public Span<byte> Data
    {
        get
        {
            unsafe
            {
                var slice = NativeMethods.sp2_bit_vec_unsafe_data_ref(Instance);
                return new Span<byte>(slice.start, (int)slice.length);
            }
        }
    }

    private unsafe BitVec(BindGen.BitVec* instance) : base(instance)
    {
    }

    private protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_bit_vec_dealloc(Instance);
    }
}
