use crate::{Connection, Packet, SendError};
use std::{error::Error, fmt::Debug};

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
#[derive(Debug)]
pub struct WebsocketConnection(
    std::sync::Mutex<
        tungstenite::WebSocket<
            tungstenite::stream::MaybeTlsStream<std::net::TcpStream>,
        >,
    >,
);

impl Connection for WebsocketConnection {
    type TransportError = tungstenite::Error;

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
            .map_err(SendError::IntoPacket)?
            .into();
        let mut socket = self.0.lock().unwrap();
        socket
            .send(tungstenite::Message::Binary(data.into()))
            .map_err(SendError::Transport)
    }
}

impl WebsocketConnection {
    /// Open a new WebSocket and connect to the provided host.
    ///
    /// Requires the feature "protocol_websocket" which is disabled by default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tungstenite::http::Uri;
    /// use servicepoint::{
    ///     Command,
    ///     connections::{WebsocketConnection as WebsocketConnection, Connection}
    /// };
    /// let uri = "ws://localhost:8080".parse().unwrap();
    /// let mut connection = WebsocketConnection::open(uri)
    ///     .expect("could not connect");
    /// connection.send(Command::Clear)
    ///     .expect("send failed");
    /// ```
    pub fn open(uri: tungstenite::http::Uri) -> tungstenite::Result<Self> {
        use tungstenite::{
            client::IntoClientRequest, connect, ClientRequestBuilder,
        };

        log::info!("connecting to {uri:?}");

        let request = ClientRequestBuilder::new(uri).into_client_request()?;
        let (sock, _) = connect(request)?;
        Ok(Self(std::sync::Mutex::new(sock)))
    }
}

impl Drop for WebsocketConnection {
    fn drop(&mut self) {
        drop(self.0.try_lock().map(move |mut sock| sock.close(None)));
    }
}
