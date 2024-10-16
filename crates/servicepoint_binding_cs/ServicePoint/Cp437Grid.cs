using System.Text;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Cp437Grid : SpNativeInstance<BindGen.Cp437Grid>
{
    public static Cp437Grid New(nuint width, nuint height)
    {
        unsafe
        {
            return new Cp437Grid(Cp437GridNative.sp_cp437_grid_new(width, height));
        }
    }

    public static Cp437Grid Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return new Cp437Grid(Cp437GridNative.sp_cp437_grid_load(width, height, bytesPtr,
                    (nuint)bytes.Length));
            }
        }
    }

    public Cp437Grid Clone()
    {
        unsafe
        {
            return new Cp437Grid(Instance->Clone());
        }
    }

    public byte this[nuint x, nuint y]
    {
        get
        {
            unsafe
            {
                return Instance->Get(x, y);
            }
        }
        set
        {
            unsafe
            {
                Instance->Set(x, y, value);
            }
        }
    }

    public string this[nuint y]
    {
        set
        {
            var width = Width;
            ArgumentOutOfRangeException.ThrowIfGreaterThan((nuint)value.Length, width);

            nuint x = 0;
            for (; x < (nuint)value.Length; x++)
                this[x, y] = (byte)value[(int)x];

            for (; x < width; x++)
                this[x, y] = 0;
        }

        get
        {
            var sb = new StringBuilder();
            for (nuint x = 0; x < Width; x++)
            {
                var val = this[x, y];
                if (val == 0)
                    break;
                sb.Append((char)val);
            }

            return sb.ToString();
        }
    }

    public void Fill(byte value)
    {
        unsafe
        {
            Instance->Fill(value);
        }
    }

    public nuint Width
    {
        get
        {
            unsafe
            {
                return Instance->Width();
            }
        }
    }

    public nuint Height
    {
        get
        {
            unsafe
            {
                return Instance->Height();
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

    private unsafe Cp437Grid(BindGen.Cp437Grid* instance) : base(instance)
    {
    }

    private protected override unsafe void Free() => Instance->Free();
}
