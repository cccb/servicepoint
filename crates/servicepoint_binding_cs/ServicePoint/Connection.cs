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
            return NativeMethods.sp_connection_send_packet(Instance, packet.Into());
        }
    }

    public bool Send(Command command)
    {
        unsafe
        {
            return NativeMethods.sp_connection_send_command(Instance, command.Into());
        }
    }

    private protected override unsafe void Free() => NativeMethods.sp_connection_free(Instance);

    private unsafe Connection(BindGen.Connection* instance) : base(instance)
    {
    }
}
