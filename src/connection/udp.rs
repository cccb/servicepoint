use crate::{Connection, Packet};
use std::fmt::Debug;
use std::net::UdpSocket;

/// A connection using the UDP protocol.
///
/// Use this when sending commands directly to the display.
///
/// Requires the feature "protocol_udp" which is enabled by default.
#[derive(Debug)]
pub struct Udp {
    socket: UdpSocket,
}

impl Udp {
    /// Open a new UDP socket and connect to the provided host.
    ///
    /// Note that this is UDP, which means that the open call can succeed even if the display is unreachable.
    ///
    /// The address of the display in CCCB is `172.23.42.29:2342`.
    ///
    /// # Errors
    ///
    /// Any errors resulting from binding the udp socket.
    ///
    /// # Examples
    /// ```rust
    ///  let connection = servicepoint::connection::Udp::open("127.0.0.1:2342")
    ///     .expect("connection failed");
    /// ```
    pub fn open(
        addr: impl std::net::ToSocketAddrs + Debug,
    ) -> std::io::Result<Self> {
        log::info!("connecting to {addr:?}");
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }
}

impl Connection for Udp {
    type Error = std::io::Error;

    fn send(&self, packet: impl Into<Packet>) -> Result<(), Self::Error> {
        let data: Vec<u8> = packet.into().into();
        self.socket.send(&data).map(move |_| ()) // ignore Ok value
    }
}

impl From<UdpSocket> for Udp {
    fn from(socket: UdpSocket) -> Self {
        Self { socket }
    }
}
