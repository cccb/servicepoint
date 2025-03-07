//! This module contains the [Connection] trait and all implementations provided in this library.

use crate::Packet;
use std::error::Error;
use std::fmt::Debug;

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

/// A connection to the display.
///
/// Used to send [Packets][Packet] or [Commands][crate::Command].
///
/// # Examples
/// ```rust
/// use servicepoint::{command, connection, Connection};
/// let connection = connection::Udp::open("127.0.0.1:2342")
///     .expect("connection failed");
///  connection.send(command::Clear)
///     .expect("send failed");
/// ```
pub trait Connection: Debug {
    /// The error that can occur when sending a packet
    type Error: Error + Debug;

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
    ///  # use servicepoint::connection::Connection;
    ///  let connection = servicepoint::connection::Fake;
    ///  // turn off all pixels on display
    ///  connection.send(servicepoint::command::Clear)
    ///      .expect("send failed");
    /// ```
    fn send(&self, packet: impl Into<Packet>) -> Result<(), Self::Error>;
}
