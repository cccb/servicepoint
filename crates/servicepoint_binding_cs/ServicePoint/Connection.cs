using System.Text;
using ServicePoint.BindGen;

namespace ServicePoint;

public sealed class Connection : SpNativeInstance<BindGen.Connection>
{
    public static Connection Open(string host)
    {
        unsafe
        {
            fixed (byte* bytePtr = Encoding.UTF8.GetBytes(host))
            {
                return new Connection(NativeMethods.sp_connection_open(bytePtr));
            }
        }
    }

    public bool Send(Packet packet)
    {
        unsafe
        {
            return NativeMethods.sp_connection_send(Instance, packet.Into());
        }
    }

    private protected override unsafe void Dealloc()
    {
        NativeMethods.sp_connection_dealloc(Instance);
    }

    private unsafe Connection(BindGen.Connection* instance) : base(instance)
    {
    }
}
