//! This module contains the [Connection] trait and all implementations provided in this library.

use crate::Packet;
use std::{error::Error, fmt::Debug};

mod fake;
#[cfg(feature = "protocol_udp")]
mod udp;
#[cfg(feature = "protocol_websocket")]
mod websocket;

pub use fake::*;
#[cfg(feature = "protocol_udp")]
pub use udp::*;
#[cfg(feature = "protocol_websocket")]
pub use websocket::*;

/// An error that can happen when sending a command
#[derive(Debug, thiserror::Error)]
pub enum SendError<
    IntoPacketError: Error + Debug,
    TransportError: Error + Debug,
> {
    #[error(transparent)]
    Transport(TransportError),
    #[error(transparent)]
    IntoPacket(IntoPacketError),
}

/// A connection to the display.
///
/// Used to send [Packets][Packet] or [Commands][crate::Command].
///
/// # Examples
/// ```rust
/// use servicepoint::{ClearCommand, Connection, UdpConnection};
/// let connection = UdpConnection::open("127.0.0.1:2342")
///     .expect("connection failed");
///  connection.send(ClearCommand)
///     .expect("send failed");
/// ```
pub trait Connection: Debug {
    /// The error that can occur when sending a packet
    type TransportError: Error + Debug;

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
    ///  # use servicepoint::Connection;
    ///  let connection = servicepoint::FakeConnection;
    ///  // turn off all pixels on display
    ///  connection.send(servicepoint::ClearCommand)
    ///      .expect("send failed");
    /// ```
    fn send<P: TryInto<Packet>>(
        &self,
        packet: P,
    ) -> Result<
        (),
        SendError<<P as TryInto<Packet>>::Error, Self::TransportError>,
    >
    where
        <P as TryInto<Packet>>::Error: Error + Debug;
}
