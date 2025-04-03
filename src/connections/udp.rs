use crate::{Connection, Packet, SendError};
use std::{
    error::Error,
    fmt::Debug,
    net::{SocketAddr, UdpSocket},
};

/// A connection using the UDP protocol.
///
/// Use this when sending commands directly to the display.
///
/// Requires the feature "`protocol_udp`" which is enabled by default.
#[derive(Debug)]
pub struct UdpConnection {
    socket: UdpSocket,
}

impl UdpConnection {
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
    ///  let connection = servicepoint::UdpConnection::open("127.0.0.1:2342")
    ///     .expect("connection failed");
    /// ```
    pub fn open(
        addr: impl std::net::ToSocketAddrs + Debug,
    ) -> std::io::Result<Self> {
        log::info!("connecting to {addr:?}");
        let socket = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 0)))?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }
}

impl Connection for UdpConnection {
    type TransportError = std::io::Error;

    fn send<P: TryInto<Packet>>(
        &self,
        packet: P,
    ) -> Result<
        (),
        SendError<<P as TryInto<Packet>>::Error, Self::TransportError>,
    >
    where
        <P as TryInto<Packet>>::Error: Error + Debug,
    {
        let data: Vec<u8> = packet
            .try_into()
            .map(Into::<Vec<u8>>::into)
            .map_err(SendError::IntoPacket)?;
        self.socket
            .send(&data)
            .map(move |_| ())
            .map_err(SendError::Transport) // ignore Ok value
    }
}

impl From<UdpSocket> for UdpConnection {
    fn from(socket: UdpSocket) -> Self {
        Self { socket }
    }
}
