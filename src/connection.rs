use crate::Packet;
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::{convert::TryInto, net::UdpSocket};

/// Allows sending commands through more types
pub trait SendCommandExt {
    /// Serializes the command and sends it through the underlying transport
    fn send_command(&self, command: impl TryInto<Packet>) -> Option<()>;
}

impl SendCommandExt for UdpSocket {
    fn send_command(&self, command: impl TryInto<Packet>) -> Option<()> {
        let packet = command.try_into().ok()?;
        let vec: Vec<_> = packet.into();
        self.send(&vec).ok()?;
        Some(())
    }
}

/// A fake connection for testing that does not actually send anything.
pub struct FakeConnection;

impl SendCommandExt for FakeConnection {
    fn send_command(&self, command: impl TryInto<Packet>) -> Option<()> {
        let packet = command.try_into().ok()?;
        drop(Vec::from(packet));
        Some(())
    }
}

/// Provides servicepoint specific extensions for `UdpSocket`
pub trait UdpSocketExt {
    /// Creates a `UdpSocket` that can be used so send to the specified addr.
    fn bind_connect(addr: impl ToSocketAddrs) -> std::io::Result<UdpSocket>;
}

impl UdpSocketExt for UdpSocket {
    fn bind_connect(addr: impl ToSocketAddrs) -> std::io::Result<UdpSocket> {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
        socket.connect(addr)?;
        Ok(socket)
    }
}
