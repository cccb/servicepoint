//! Abstractions for the UDP protocol of the CCCB servicepoint display.
//!
//! Your starting point is a [Connection] to the display.
//! With a connection, you can send [Command]s.
//! When received, the display will update the state of the pixels.
//!
//! # Examples
//!
//! ```rust
//! use servicepoint::{Command, CompressionCode, Grid, Bitmap};
//!
//! let connection = servicepoint::Connection::open("127.0.0.1:2342")
//!     .expect("connection failed");
//!
//!  // turn off all pixels on display
//!  connection.send(Command::Clear)
//!     .expect("send failed");
//! ```
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

use std::time::Duration;

pub use bitvec;

pub use crate::bitmap::Bitmap;
pub use crate::brightness::{Brightness, BrightnessGrid};
pub use crate::command::{Command, Offset};
pub use crate::compression_code::CompressionCode;
pub use crate::connection::Connection;
pub use crate::cp437::{CharGrid, Cp437Grid};
pub use crate::data_ref::DataRef;
pub use crate::grid::Grid;
pub use crate::origin::{Origin, Pixels, Tiles};
pub use crate::primitive_grid::PrimitiveGrid;

/// An alias for the specific type of [bitvec::prelude::BitVec] used.
pub type BitVec = bitvec::prelude::BitVec<u8, bitvec::prelude::Msb0>;

mod bitmap;
mod brightness;
mod command;
mod command_code;
mod compression;
mod compression_code;
mod connection;
pub mod cp437;
mod data_ref;
mod grid;
mod origin;
pub mod packet;
mod primitive_grid;

/// size of a single tile in one dimension
pub const TILE_SIZE: usize = 8;

/// Display tile count in the x-direction
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Cp437Grid, TILE_HEIGHT, TILE_WIDTH};
/// let grid = Cp437Grid::new(TILE_WIDTH, TILE_HEIGHT);
/// ```
pub const TILE_WIDTH: usize = 56;

/// Display tile count in the y-direction
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Cp437Grid, TILE_HEIGHT, TILE_WIDTH};
/// let grid = Cp437Grid::new(TILE_WIDTH, TILE_HEIGHT);
/// ```
pub const TILE_HEIGHT: usize = 20;

/// Display width in pixels
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{PIXEL_HEIGHT, PIXEL_WIDTH, Bitmap};
/// let grid = Bitmap::new(PIXEL_WIDTH, PIXEL_HEIGHT);
/// ```
pub const PIXEL_WIDTH: usize = TILE_WIDTH * TILE_SIZE;

/// Display height in pixels
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{PIXEL_HEIGHT, PIXEL_WIDTH, Bitmap};
/// let grid = Bitmap::new(PIXEL_WIDTH, PIXEL_HEIGHT);
/// ```
pub const PIXEL_HEIGHT: usize = TILE_HEIGHT * TILE_SIZE;

/// pixel count on whole screen
pub const PIXEL_COUNT: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

/// Actual hardware limit is around 28-29ms/frame. Rounded up for less dropped packets.
///
/// # Examples
///
/// ```rust
/// # use std::time::Instant;
/// # use servicepoint::{Command, CompressionCode, FRAME_PACING, Origin, Bitmap};
/// # let connection = servicepoint::Connection::Fake;
/// # let pixels = Bitmap::max_sized();
/// loop {
///    let start = Instant::now();
///
///    // Change pixels here
///
///    connection.send(Command::BitmapLinearWin(
///            Origin::new(0,0),
///            pixels,
///            CompressionCode::Lzma
///        ))
///        .expect("send failed");
///
///    // warning: will crash if resulting duration is negative, e.g. when resuming from standby
///    std::thread::sleep(FRAME_PACING - start.elapsed());
///    # break; // prevent doctest from hanging
/// }
/// ```
pub const FRAME_PACING: Duration = Duration::from_millis(30);

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;
