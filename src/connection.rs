use std::net::{ToSocketAddrs, UdpSocket};
use crate::{Packet};

pub struct Connection {
    socket: UdpSocket,
}

pub trait ToPacket {
    fn to_packet(self) -> Packet;
}

impl Connection {
    /// Open a new UDP socket and create a display instance
    pub fn open(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }

    /// Send a command to the display
    pub fn send(&self, packet: impl ToPacket) -> std::io::Result<()> {
        let packet = packet.to_packet();
        self.socket.send(&*packet.to_bytes())?;
        Ok(())
    }
}
