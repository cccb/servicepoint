using System.Text;

namespace ServicePoint;

public sealed partial class Connection
{
    public static Connection? Open(string host)
    {
        unsafe
        {
            fixed (byte* bytePtr = Encoding.UTF8.GetBytes(host))
            {
                return Open(bytePtr);
            }
        }
    }

    public bool Send(Packet packet) => SendPacket(packet);

    public bool Send(Command command) => SendCommand(command);
}
