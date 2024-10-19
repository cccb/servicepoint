using System.Text;

namespace ServicePoint;

public sealed partial class Cp437Grid
{
    public static Cp437Grid Load(nuint width, nuint height, Span<byte> bytes)
    {
        unsafe
        {
            fixed (byte* bytesPtr = bytes)
            {
                return Load(width, height, bytesPtr, (nuint)bytes.Length);
            }
        }
    }

    public byte this[nuint x, nuint y]
    {
        get => Get(x, y);
        set => Set(x, y, value);
    }

    public string this[nuint y]
    {
        set
        {
            var width = Width();
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
            var width = Width();
            for (nuint x = 0; x < width; x++)
            {
                var val = this[x, y];
                if (val == 0)
                    break;
                sb.Append((char)val);
            }

            return sb.ToString();
        }
    }

    public Span<byte> Data => UnsafeDataRef().AsSpan();
}
