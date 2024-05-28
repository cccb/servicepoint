//! C API wrapper for the `servicepoint` crate.

pub use servicepoint::{
    CompressionCode, PIXEL_COUNT, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT,
    TILE_SIZE, TILE_WIDTH,
};

pub use crate::c_slice::CByteSlice;

/// C functions for interacting with `BitVec`s
///
/// prefix `sp_bit_vec_`
pub mod bit_vec;

/// C functions for interacting with `ByteGrid`s
///
/// prefix `sp_byte_grid_`
pub mod byte_grid;

/// C functions for interacting with `Command`s
///
/// prefix `sp_command_`
pub mod command;

/// C functions for interacting with `Connection`s
///
/// prefix `sp_connection_`
pub mod connection;

/// C functions for interacting with `Packet`s
///
/// prefix `sp_packet_`
pub mod packet;

/// C functions for interacting with `PixelGrid`s
///
/// prefix `sp_pixel_grid_`
pub mod pixel_grid;

/// The minimum time needed for the display to refresh the screen in ms.
pub const FRAME_PACING_MS: u32 = servicepoint::FRAME_PACING.as_millis() as u32;

mod c_slice;
