use std::fmt::Debug;
use std::net::{ToSocketAddrs, UdpSocket};

use log::{debug, info};

use crate::Packet;

/// A connection to the display.
pub struct Connection {
    socket: UdpSocket,
}

impl Connection {
    /// Open a new UDP socket and connect to the provided host.
    ///
    /// Note that this is UDP, which means that the open call can succeed even if the display is unreachable.
    ///
    /// # Examples
    /// ```rust
    ///  let connection = servicepoint2::Connection::open("172.23.42.29:2342")
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
    /// # Examples
    ///
    /// ```rust
    ///  let connection = servicepoint2::Connection::open("172.23.42.29:2342")
    ///     .expect("connection failed");
    ///
    ///  // turn off all pixels
    ///  connection.send(servicepoint2::Command::Clear)
    ///     .expect("send failed");
    ///
    ///  // turn on all pixels
    ///  let mut pixels = servicepoint2::PixelGrid::max_sized();
    ///  pixels.fill(true);
    ///
    ///  // send pixels to display
    ///  connection.send(servicepoint2::Command::BitmapLinearWin(servicepoint2::Origin::top_left(), pixels))
    ///     .expect("send failed");
    /// ```
    pub fn send(
        &self,
        packet: impl Into<Packet> + Debug,
    ) -> Result<(), std::io::Error> {
        debug!("sending {packet:?}");
        let packet: Packet = packet.into();
        let data: Vec<u8> = packet.into();
        self.socket.send(&*data)?;
        Ok(())
    }
}
