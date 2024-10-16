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
                return new Connection(ConnectionNative.sp_connection_open(bytePtr));
            }
        }
    }

    public bool Send(Packet packet)
    {
        unsafe
        {
            return Instance->SendPacket(packet.Into());
        }
    }

    public bool Send(Command command)
    {
        unsafe
        {
            return Instance->SendCommand(command.Into());
        }
    }

    private protected override unsafe void Free() => Instance->Free();

    private unsafe Connection(BindGen.Connection* instance) : base(instance)
    {
    }
}
