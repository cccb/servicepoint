use crate::packet::Packet;
use std::fmt::Debug;

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
        std::sync::Mutex<
            tungstenite::WebSocket<
                tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
            >,
        >,
    ),

    /// A fake connection for testing that does not actually send anything.
    Fake,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("IO error occurred while sending")]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "protocol_websocket")]
    #[error("WebSocket error occurred while sending")]
    WebsocketError(#[from] tungstenite::Error),
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
    /// ```no_run
    /// use tungstenite::http::Uri;
    /// use servicepoint::{Command, Connection};
    /// let uri = "ws://localhost:8080".parse().unwrap();
    /// let mut connection = Connection::open_websocket(uri)
    ///     .expect("could not connect");
    /// connection.send(Command::Clear)
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
        Ok(Self::WebSocket(std::sync::Mutex::new(
            sock,
        )))
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
            #[cfg(feature = "protocol_websocket")]
            Connection::WebSocket(socket) => {
                let mut socket = socket.lock().unwrap();
                socket
                    .send(tungstenite::Message::Binary(data.into()))
                    .map_err(SendError::WebsocketError)
            }
            Connection::Fake => {
                let _ = data;
                Ok(())
            }
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        #[cfg(feature = "protocol_websocket")]
        if let Connection::WebSocket(sock) = self {
            _ = sock
                .try_lock()
                .map(move |mut sock| sock.close(None));
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
