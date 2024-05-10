mod connection;
mod pixel_grid;
mod bit_vec;
mod packet;
mod command;
mod command_codes;

pub use crate::connection::{Connection, ToPacket};
pub use crate::pixel_grid::{PixelGrid};
pub use crate::bit_vec::{BitVec};
pub use crate::packet::{Packet, Header, Payload};
pub use crate::command::{Command, Size, Origin, Window};
pub use crate::command_codes::{DisplayCommandCode};

pub const TILE_SIZE: u16 = 8;
pub const TILE_WIDTH: u16 = 56;
pub const TILE_HEIGHT: u16 = 20;
pub const PIXEL_WIDTH: u16 = TILE_WIDTH * TILE_SIZE;
pub const PIXEL_HEIGHT: u16 = TILE_HEIGHT * TILE_SIZE;
pub const PIXEL_COUNT: usize = PIXEL_WIDTH as usize * PIXEL_HEIGHT as usize;
