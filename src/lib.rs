pub use crate::bit_vec::BitVec;
pub use crate::byte_grid::ByteGrid;
pub use crate::command::{Command, Origin, Size};
pub use crate::command_codes::{CommandCode, CompressionCode};
pub use crate::connection::Connection;
pub use crate::packet::{Header, Packet, Payload};
pub use crate::pixel_grid::PixelGrid;

mod bit_vec;
mod byte_grid;
mod command;
mod command_codes;
mod compression;
mod connection;
mod packet;
mod pixel_grid;

pub const TILE_SIZE: u16 = 8;
pub const TILE_WIDTH: u16 = 56;
pub const TILE_HEIGHT: u16 = 20;
pub const PIXEL_WIDTH: u16 = TILE_WIDTH * TILE_SIZE;
pub const PIXEL_HEIGHT: u16 = TILE_HEIGHT * TILE_SIZE;
pub const PIXEL_COUNT: usize = PIXEL_WIDTH as usize * PIXEL_HEIGHT as usize;
