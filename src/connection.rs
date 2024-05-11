use std::fmt::Debug;
use std::net::{ToSocketAddrs, UdpSocket};
use log::{debug, info};
use crate::Packet;

pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    /// Open a new UDP socket and create a display instance
    pub fn open(addr: impl ToSocketAddrs + Debug) -> std::io::Result<Self> {
        info!("connecting to {addr:?}");
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }

    /// Send a command to the display
    pub fn send(&self, packet: impl Into<Packet> + Debug) -> std::io::Result<()> {
        debug!("sending {packet:?}");
        let packet = packet.into();
        let data: Vec<u8> = packet.into();
        self.socket.send(&*data)?;
        Ok(())
    }
}
