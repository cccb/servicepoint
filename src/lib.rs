//! Abstractions for the UDP protocol of the CCCB servicepoint display.
//!
//! Your starting point is a [`std::net::UdpSocket`] connected to the display.
//! With a socket, you can send [Command]s.
//! When received, the display will update the state of its pixels.
//!
//! # Examples
//!
//! ### Clear display
//!
//! ```rust
//! use std::net::UdpSocket;
//! use servicepoint::*;
//!
//! // establish a connection
//! let connection = UdpSocket::bind_connect("127.0.0.1:2342")
//!     .expect("connection failed");
//!
//!  # let connection = FakeConnection; // do not fail tests
//!  // turn off all pixels on display
//!  connection.send_command(ClearCommand)
//!     .expect("send failed");
//! ```
//!
//! ### Set all pixels to on
//!
//! ```rust
//! # use std::net::UdpSocket;
//! # use servicepoint::*;
//! # let connection = FakeConnection;
//!  // turn on all pixels in a grid
//!  let mut pixels = Bitmap::max_sized();
//!  pixels.fill(true);
//!
//!  // create command to send pixels
//!  let command = BitmapCommand {
//!     origin: Origin::ZERO,
//!     bitmap: pixels,
//!     compression: CompressionCode::default()
//!  };
//!
//!  // send command to display
//!  connection.send_command(command).expect("send failed");
//! ```
//!
//! ### Send text
//!
//! ```rust
//! # use std::net::UdpSocket;
//! # use servicepoint::*;
//! # let connection = FakeConnection;
//! // create a text grid
//! let mut grid = CharGrid::from("Hello\nCCCB?");
//! // modify the grid
//! grid.set(grid.width() - 1, 1, '!');
//! // create the command to send the data
//! let command = CharGridCommand { origin: Origin::ZERO, grid };
//! // send command to display
//! connection.send_command(command).expect("send failed");
//! ```
//!
//! ### Convert a packet to a command and back
//!
//! ```rust
//! use servicepoint::{Command, Packet, TypedCommand};
//! # let command = servicepoint::ClearCommand;
//! let packet: Packet = command.into();
//! let command = TypedCommand::try_from(packet).expect("could not read command from packet");
//! ```
//!
//! ### Convert a packet to bytes and back
//!
//! ```rust
//! use servicepoint::{Command, Packet};
//! # let command = servicepoint::ClearCommand;
//! # let packet: Packet = command.into();
//! let bytes: Vec<u8> = packet.into();
//! let packet = Packet::try_from(bytes).expect("could not read packet from bytes");
//! ```

pub use crate::brightness::Brightness;
pub use crate::command_code::CommandCode;
pub use crate::commands::*;
pub use crate::compression_code::CompressionCode;
pub use crate::connection::*;
pub use crate::constants::*;
pub use crate::containers::*;
pub use crate::origin::{Origin, Pixels, Tiles};
pub use crate::packet::{Header, Packet, Payload};

mod brightness;
mod command_code;
mod commands;
mod compression;
mod compression_code;
mod connection;
mod constants;
mod containers;
#[cfg(feature = "cp437")]
pub mod cp437;
mod origin;
mod packet;

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = usize;
