pub use crate::bit_vec::BitVec;
pub use crate::byte_grid::ByteGrid;
pub use crate::command::{Command, Origin, Size};
pub use crate::connection::Connection;
pub use crate::packet::{Header, Packet, Payload};
pub use crate::pixel_grid::PixelGrid;
pub use crate::command_code::CommandCode;
pub use crate::compression_code::CompressionCode;

mod bit_vec;
mod byte_grid;
mod command;
mod command_code;
mod compression;
mod connection;
mod packet;
mod pixel_grid;
mod compression_code;

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
