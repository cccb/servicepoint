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
//! use servicepoint::{Connection, Command};
//!
//! // establish a connection
//! let connection = Connection::open("127.0.0.1:2342")
//!     .expect("connection failed");
//!
//!  // turn off all pixels on display
//!  connection.send(Command::Clear)
//!     .expect("send failed");
//! ```
//!
//! ### Set all pixels to on
//!
//! ```rust
//! # use servicepoint::{Command, CompressionCode, Grid, Bitmap};
//! # let connection = servicepoint::Connection::open("127.0.0.1:2342").expect("connection failed");
//!  // turn on all pixels in a grid
//!  let mut pixels = Bitmap::max_sized();
//!  pixels.fill(true);
//!
//!  // create command to send pixels
//!  let command = Command::BitmapLinearWin(
//!     servicepoint::Origin::ZERO,
//!     pixels,
//!     CompressionCode::Uncompressed
//!  );
//!
//!  // send command to display
//!  connection.send(command).expect("send failed");
//! ```
//!
//! ### Send text
//!
//! ```rust
//! # use servicepoint::{Command, CompressionCode, Grid, Bitmap, CharGrid};
//! # let connection = servicepoint::Connection::open("127.0.0.1:2342").expect("connection failed");
//! // create a text grid
//! let mut grid = CharGrid::from("Hello\nCCCB?");
//! // modify the grid
//! grid.set(grid.width() - 1, 1, '!');
//! // create the command to send the data
//! let command = Command::Utf8Data(servicepoint::Origin::ZERO, grid);
//! // send command to display
//! connection.send(command).expect("send failed");
//! ```

pub use crate::bit_vec::{bitvec, BitVec};
pub use crate::bitmap::Bitmap;
pub use crate::brightness::Brightness;
pub use crate::brightness_grid::BrightnessGrid;
pub use crate::byte_grid::ByteGrid;
pub use crate::char_grid::CharGrid;
pub use crate::command::{Command, Offset};
pub use crate::compression_code::CompressionCode;
pub use crate::connection::Connection;
pub use crate::constants::*;
pub use crate::cp437::Cp437Converter;
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
mod command;
mod command_code;
mod compression;
mod compression_code;
mod connection;
mod constants;
mod cp437_grid;
mod data_ref;
mod grid;
mod origin;
mod packet;
mod value_grid;

#[cfg(feature = "cp437")]
mod cp437;

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;
