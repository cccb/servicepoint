using System.Text;
using ServicePoint2.BindGen;

namespace ServicePoint2;

public sealed class Connection : Sp2NativeInstance<BindGen.Connection>
{
    public static Connection Open(string host)
    {
        unsafe
        {
            fixed (byte* bytePtr = Encoding.UTF8.GetBytes(host))
            {
                return new Connection(NativeMethods.sp2_connection_open(bytePtr));
            }
        }
    }

    public bool Send(Command command)
    {
        unsafe
        {
            return NativeMethods.sp2_connection_send(Instance, command.Into());
        }
    }

    protected override unsafe void Dealloc()
    {
        NativeMethods.sp2_connection_dealloc(Instance);
    }

    private unsafe Connection(BindGen.Connection* instance) : base(instance)
    {
    }
}
