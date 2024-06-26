use std::fmt::Debug;
use std::net::{ToSocketAddrs, UdpSocket};

use log::{debug, info};

use crate::Packet;

/// A connection to the display.
///
/// # Examples
/// ```rust
/// # use servicepoint::Command;
/// let connection = servicepoint::Connection::open("172.23.42.29:2342")
///     .expect("connection failed");
///  connection.send(Command::Clear)
///     .expect("send failed");
/// ```
pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
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

    /// Send something packet-like to the display. Usually this is in the form of a Command.
    ///
    /// # Arguments
    ///
    /// * `packet`: the packet-like to send
    ///
    /// returns: Ok if packet was sent, otherwise socket error
    ///
    /// # Errors
    ///
    /// Any errors produced while sending using the underlying socket.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, CompressionCode, Grid, PixelGrid};
    /// # let connection = servicepoint::Connection::open("172.23.42.29:2342")
    /// #     .expect("connection failed");
    ///  // turn off all pixels on display
    ///  connection.send(Command::Clear)
    ///     .expect("send failed");
    /// ```
    pub fn send(
        &self,
        packet: impl Into<Packet>,
    ) -> Result<(), std::io::Error> {
        let packet = packet.into();
        debug!("sending {packet:?}");
        let data: Vec<u8> = packet.into();
        self.socket.send(&data)?;
        Ok(())
    }
}
