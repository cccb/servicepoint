use std::fmt::Debug;
use std::net::{ToSocketAddrs, UdpSocket};

use log::{debug, info};

use crate::Packet;

/// A connection to the display.
///
/// # Examples
/// ```rust
/// # use servicepoint::{Command, Connection};
/// let connection = servicepoint::UdpConnection::open("172.23.42.29:2342")
///     .expect("connection failed");
///  connection.send(Command::Clear);
/// ```
pub trait Connection {
    /// Send something packet-like to the display. Usually this is in the form of a Command.
    ///
    /// # Arguments
    ///
    /// - `packet`: the packet-like to send
    ///
    /// returns: true if packet was sent, otherwise false
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection };
    /// # let connection = servicepoint::UdpConnection::open("172.23.42.29:2342")
    /// #     .expect("connection failed");
    ///  // turn off all pixels on display
    ///  connection.send(Command::Clear);
    /// ```
    fn send(&self, packet: impl Into<Packet>) -> bool;
}

/// A real connection using the UDP protocol
pub struct UdpConnection {
    socket: UdpSocket,
}

impl UdpConnection {
    /// Open a new UDP socket and connect to the provided host.
    ///
    /// Note that this is UDP, which means that the open call can succeed even if the display is unreachable.
    ///
    /// # Errors
    ///
    /// Any errors resulting from binding the udp socket.
    ///
    /// # Examples
    /// ```rust
    ///  let connection = servicepoint::Connection::open("172.23.42.29:2342")
    ///     .expect("connection failed");
    /// ```
    pub fn open(addr: impl ToSocketAddrs + Debug) -> std::io::Result<Self> {
        info!("connecting to {addr:?}");
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self { socket })
    }
}

impl Connection for UdpConnection {
    fn send(&self, packet: impl Into<Packet>) -> bool {
        let packet = packet.into();
        debug!("sending {packet:?}");
        let data: Vec<u8> = packet.into();
        self.socket.send(&data).is_err()
    }
}

/// A fake connection for testing that does not actually send anything
pub struct NoopConnection;

impl Connection for NoopConnection {
    fn send(&self, packet: impl Into<Packet>) -> bool {
        true
    }
}
