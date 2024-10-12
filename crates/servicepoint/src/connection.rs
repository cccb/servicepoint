use std::fmt::Debug;

use crate::packet::Packet;

/// A connection to the display.
///
/// Used to send [Packets][Packet] or [Commands][crate::Command].
///
/// # Examples
/// ```rust
/// let connection = servicepoint::Connection::open("127.0.0.1:2342")
///     .expect("connection failed");
///  connection.send(servicepoint::Command::Clear)
///     .expect("send failed");
/// ```
#[derive(Debug)]
pub enum Connection {
    /// A connection using the UDP protocol.
    ///
    /// Use this when sending commands directly to the display.
    ///
    /// Requires the feature "protocol_udp" which is enabled by default.
    #[cfg(feature = "protocol_udp")]
    Udp(std::net::UdpSocket),

    /// A connection using the WebSocket protocol.
    ///
    /// Note that you will need to forward the WebSocket messages via UDP to the display.
    /// You can use [servicepoint-websocket-relay] for this.
    ///
    /// To create a new WebSocket automatically, use [Connection::open_websocket].
    ///
    /// Requires the feature "protocol_websocket" which is disabled by default.
    ///
    /// [servicepoint-websocket-relay]: https://github.com/kaesaecracker/servicepoint-websocket-relay
    #[cfg(feature = "protocol_websocket")]
    WebSocket(
        tungstenite::WebSocket<
            tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
        >,
    ),

    /// A fake connection for testing that does not actually send anything.
    ///
    /// This variant allows immutable send.
    Fake,

    /// A fake connection for testing that does not actually send anything.
    ///
    /// This variant does not allow immutable send.
    FakeMutableSend,
}

#[derive(Debug)]
pub enum SendError {
    IoError(std::io::Error),
    #[cfg(feature = "protocol_websocket")]
    WebsocketError(tungstenite::Error),
}

impl Connection {
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
    ///  let connection = servicepoint::Connection::open("127.0.0.1:2342")
    ///     .expect("connection failed");
    /// ```
    #[cfg(feature = "protocol_udp")]
    pub fn open(
        addr: impl std::net::ToSocketAddrs + Debug,
    ) -> std::io::Result<Self> {
        log::info!("connecting to {addr:?}");
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        Ok(Self::Udp(socket))
    }

    /// Open a new WebSocket and connect to the provided host.
    ///
    /// Requires the feature "protocol_websocket" which is disabled by default.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tungstenite::http::Uri;
    /// use servicepoint::{Command, Connection};
    /// let uri = "ws://localhost:8080".parse().unwrap();
    /// let mut connection = Connection::open_websocket(uri)
    ///     .expect("could not connect");
    /// connection.send_mut(Command::Clear)
    ///     .expect("send failed");
    /// ```
    #[cfg(feature = "protocol_websocket")]
    pub fn open_websocket(
        uri: tungstenite::http::Uri,
    ) -> tungstenite::Result<Self> {
        use tungstenite::{
            client::IntoClientRequest, connect, ClientRequestBuilder,
        };

        log::info!("connecting to {uri:?}");

        let request = ClientRequestBuilder::new(uri).into_client_request()?;
        let (sock, _) = connect(request)?;

        Ok(Self::WebSocket(sock))
    }

    /// Send something packet-like to the display. Usually this is in the form of a Command.
    ///
    /// This variant can only be used for connections that support immutable send, e.g. [Connection::Udp].
    ///
    /// If you want to be able to switch the protocol, you should use [Self::send_mut] instead.
    ///
    /// # Arguments
    ///
    /// - `packet`: the packet-like to send
    ///
    /// returns: true if packet was sent, otherwise false
    ///
    /// # Panics
    ///
    /// If the connection does not support immutable send, e.g. for [Connection::WebSocket].
    ///
    /// # Examples
    ///
    /// ```rust
    ///  let connection = servicepoint::Connection::Fake;
    ///  // turn off all pixels on display
    ///  connection.send(servicepoint::Command::Clear)
    ///      .expect("send failed");
    /// ```
    pub fn send(&self, packet: impl Into<Packet>) -> Result<(), SendError> {
        let packet = packet.into();
        log::debug!("sending {packet:?}");
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
            #[allow(unreachable_patterns)] // depends on features
            _ => {
                panic!("Connection {:?} does not support immutable send", self)
            }
        }
    }

    /// Send something packet-like to the display. Usually this is in the form of a Command.
    ///
    /// This variant has to be used for connections that do not support immutable send, e.g. [Connection::WebSocket].
    ///
    /// If you want to be able to switch the protocol, you should use this variant.
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
    ///  let mut connection = servicepoint::Connection::FakeMutableSend;
    ///  // turn off all pixels on display
    ///  connection.send_mut(servicepoint::Command::Clear)
    ///      .expect("send failed");
    /// ```
    pub fn send_mut(
        &mut self,
        packet: impl Into<Packet>,
    ) -> Result<(), SendError> {
        match self {
            #[cfg(feature = "protocol_websocket")]
            Connection::WebSocket(socket) => {
                let packet = packet.into();
                log::debug!("sending {packet:?}");
                let data: Vec<u8> = packet.into();
                socket
                    .send(tungstenite::Message::Binary(data))
                    .map_err(SendError::WebsocketError)
            }
            Connection::FakeMutableSend => {
                let packet = packet.into();
                log::debug!("sending {packet:?}");
                let data: Vec<u8> = packet.into();
                let _ = data;
                Ok(())
            }
            _ => self.send(packet),
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

    #[test]
    fn send_fake_mutable() {
        let data: &[u8] = &[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let packet = Packet::try_from(data).unwrap();
        Connection::FakeMutableSend.send_mut(packet).unwrap()
    }

    #[test]
    #[should_panic]
    fn send_fake_mutable_panic() {
        let data: &[u8] = &[0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let packet = Packet::try_from(data).unwrap();
        Connection::FakeMutableSend.send(packet).unwrap()
    }
}
