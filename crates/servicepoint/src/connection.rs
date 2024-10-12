use std::fmt::Debug;

use log::debug;

#[cfg(feature = "protocol_udp")]
use log::info;
#[cfg(feature = "protocol_udp")]
use std::net::{ToSocketAddrs, UdpSocket};

use crate::packet::Packet;

/// A connection to the display.
///
/// Used to send [Packets][Packet] or [Commands][crate::Command].
///
/// # Examples
/// ```rust
/// let connection = servicepoint::Connection::open("172.23.42.29:2342")
///     .expect("connection failed");
///  connection.send(servicepoint::Command::Clear)
///     .expect("send failed");
/// ```
pub enum Connection {
    /// A real connection using the UDP protocol
    #[cfg(feature = "protocol_udp")]
    Udp(UdpSocket),
    /// A fake connection for testing that does not actually send anything.
    Fake,
}

#[derive(Debug)]
pub enum SendError {
    IoError(std::io::Error),
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
    #[cfg(feature = "protocol_udp")]
    pub fn open(addr: impl ToSocketAddrs + Debug) -> std::io::Result<Self> {
        info!("connecting to {addr:?}");
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self::Udp(socket))
    }

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
    /// # let connection = servicepoint::Connection::Fake;
    ///  // turn off all pixels on display
    ///  connection.send(servicepoint::Command::Clear)
    ///      .expect("send failed");
    /// ```
    pub fn send(&self, packet: impl Into<Packet>) -> Result<(), SendError> {
        let packet = packet.into();
        debug!("sending {packet:?}");
        let data: Vec<u8> = packet.into();
        match self {
            #[cfg(feature = "protocol_udp")]
            Connection::Udp(socket) => {
                socket
                    .send(&data)
                    .map_err(SendError::IoError)
                    .map(move |_| ()) // ignore Ok value
            }
            Connection::Fake => {
                let _ = data;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::*;

    #[test]
    fn send_fake() {
        let data: &[u8] = &[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let packet = Packet::try_from(data).unwrap();
        Connection::Fake.send(packet).unwrap()
    }
}
