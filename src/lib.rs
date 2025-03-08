//! Abstractions for the UDP protocol of the CCCB servicepoint display.
//!
//! Your starting point is a [Connection] to the display.
//! With a connection, you can send [Command]s.
//! When received, the display will update the state of its pixels.
//!
//! # Examples
//!
//! ### Clear display
//!
//! ```rust
//! use servicepoint::*;
//!
//! // establish a connection
//! let connection = UdpConnection::open("127.0.0.1:2342")
//!     .expect("connection failed");
//!
//!  // turn off all pixels on display
//!  connection.send(ClearCommand)
//!     .expect("send failed");
//! ```
//!
//! ### Set all pixels to on
//!
//! ```rust
//! # use servicepoint::*;
//! # let connection = UdpConnection::open("127.0.0.1:2342").expect("connection failed");
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
//!  connection.send(command).expect("send failed");
//! ```
//!
//! ### Send text
//!
//! ```rust
//! # use servicepoint::*;
//! # let connection = UdpConnection::open("127.0.0.1:2342").expect("connection failed");
//! // create a text grid
//! let mut grid = CharGrid::from("Hello\nCCCB?");
//! // modify the grid
//! grid.set(grid.width() - 1, 1, '!');
//! // create the command to send the data
//! let command = CharGridCommand { origin: Origin::ZERO, grid };
//! // send command to display
//! connection.send(command).expect("send failed");
//! ```

pub use crate::brightness::Brightness;
pub use crate::commands::*;
pub use crate::compression_code::CompressionCode;
pub use crate::connections::*;
pub use crate::constants::*;
pub use crate::containers::*;
pub use crate::origin::{Origin, Pixels, Tiles};
pub use crate::packet::{Header, Packet, Payload};

mod brightness;
mod commands;
mod command_code;
mod compression;
mod compression_code;
mod connections;
mod constants;
mod containers;
mod origin;
mod packet;
#[cfg(feature = "cp437")]
mod cp437;

#[cfg(feature = "cp437")]
pub use crate::cp437::Cp437Converter;

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = usize;
