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
//! let connection = connection::Udp::open("127.0.0.1:2342")
//!     .expect("connection failed");
//!
//!  // turn off all pixels on display
//!  connection.send(command::Clear)
//!     .expect("send failed");
//! ```
//!
//! ### Set all pixels to on
//!
//! ```rust
//! # use servicepoint::*;
//! # let connection = connection::Udp::open("127.0.0.1:2342").expect("connection failed");
//!  // turn on all pixels in a grid
//!  let mut pixels = Bitmap::max_sized();
//!  pixels.fill(true);
//!
//!  // create command to send pixels
//!  let command = command::BitmapLinearWin {
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
//! # let connection = connection::Udp::open("127.0.0.1:2342").expect("connection failed");
//! // create a text grid
//! let mut grid = CharGrid::from("Hello\nCCCB?");
//! // modify the grid
//! grid.set(grid.width() - 1, 1, '!');
//! // create the command to send the data
//! let command = command::Utf8Data { origin: Origin::ZERO, grid };
//! // send command to display
//! connection.send(command).expect("send failed");
//! ```

pub use crate::bit_vec::{bitvec, BitVec};
pub use crate::bitmap::Bitmap;
pub use crate::brightness::Brightness;
pub use crate::brightness_grid::BrightnessGrid;
pub use crate::byte_grid::ByteGrid;
pub use crate::char_grid::CharGrid;
pub use crate::command::{Command, TypedCommand};
pub use crate::compression_code::CompressionCode;
pub use crate::connection::Connection;
pub use crate::constants::*;
pub use crate::cp437_grid::Cp437Grid;
pub use crate::data_ref::DataRef;
pub use crate::grid::Grid;
pub use crate::origin::{Origin, Pixels, Tiles};
pub use crate::packet::{Header, Packet, Payload};
pub use crate::value_grid::{
    IterGridRows, SetValueSeriesError, TryLoadValueGridError, Value, ValueGrid,
};

mod bit_vec;
mod bitmap;
mod brightness;
mod brightness_grid;
mod byte_grid;
mod char_grid;
pub mod command;
mod command_code;
mod compression;
mod compression_code;
pub mod connection;
mod constants;
mod cp437_grid;
mod data_ref;
mod grid;
mod origin;
mod packet;
mod value_grid;

#[cfg(feature = "cp437")]
mod cp437;
mod parser;

#[cfg(feature = "cp437")]
pub use crate::cp437::Cp437Converter;

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = usize;
