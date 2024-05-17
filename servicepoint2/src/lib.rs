pub use crate::bit_vec::BitVec;
pub use crate::byte_grid::ByteGrid;
pub use crate::command::{Brightness, Command, Offset, Origin, Size};
pub use crate::compression_code::CompressionCode;
pub use crate::connection::Connection;
pub use crate::packet::{Header, Packet, Payload};
pub use crate::pixel_grid::PixelGrid;
use std::time::Duration;

#[cfg(feature = "c_api")]
pub use crate::c_slice::CByteSlice;

mod bit_vec;
mod byte_grid;
mod c_slice;
mod command;
mod command_code;
mod compression;
mod compression_code;
mod connection;
mod packet;
mod pixel_grid;

/// size of a single tile in one dimension
pub const TILE_SIZE: u16 = 8;
/// tile count in the x-direction
pub const TILE_WIDTH: u16 = 56;
/// tile count in the y-direction
pub const TILE_HEIGHT: u16 = 20;
/// screen width in pixels
pub const PIXEL_WIDTH: u16 = TILE_WIDTH * TILE_SIZE;
/// screen height in pixels
pub const PIXEL_HEIGHT: u16 = TILE_HEIGHT * TILE_SIZE;
/// pixel count on whole screen
pub const PIXEL_COUNT: usize = PIXEL_WIDTH as usize * PIXEL_HEIGHT as usize;

/// Actual hardware limit is around 28-29ms/frame. Rounded up for less dropped packets.
pub const FRAME_PACING: Duration = Duration::from_millis(30);
