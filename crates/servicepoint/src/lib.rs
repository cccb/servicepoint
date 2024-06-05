//! Abstractions for the UDP protocol of the CCCB servicepoint display.

use std::time::Duration;

pub use bitvec;
use bitvec::prelude::{BitVec, Msb0};

pub use crate::byte_grid::ByteGrid;
pub use crate::command::{Brightness, Command, Offset, Origin};
pub use crate::compression_code::CompressionCode;
pub use crate::connection::Connection;
pub use crate::data_ref::DataRef;
pub use crate::grid::Grid;
pub use crate::packet::{Header, Packet, Payload};
pub use crate::pixel_grid::PixelGrid;

type SpBitVec = BitVec<u8, Msb0>;

mod byte_grid;
mod command;
mod command_code;
mod compression;
mod compression_code;
mod connection;
mod data_ref;
mod grid;
mod packet;
mod pixel_grid;

/// size of a single tile in one dimension
pub const TILE_SIZE: usize = 8;

/// tile count in the x-direction
pub const TILE_WIDTH: usize = 56;

/// tile count in the y-direction
pub const TILE_HEIGHT: usize = 20;

/// screen width in pixels
pub const PIXEL_WIDTH: usize = TILE_WIDTH * TILE_SIZE;

/// screen height in pixels
pub const PIXEL_HEIGHT: usize = TILE_HEIGHT * TILE_SIZE;

/// pixel count on whole screen
pub const PIXEL_COUNT: usize = PIXEL_WIDTH * PIXEL_HEIGHT;

/// Actual hardware limit is around 28-29ms/frame. Rounded up for less dropped packets.
pub const FRAME_PACING: Duration = Duration::from_millis(30);

// include README.md in doctest
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;
