//! C API wrapper for the `servicepoint` crate.

pub use servicepoint::{
    CompressionCode, PIXEL_COUNT, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT,
    TILE_SIZE, TILE_WIDTH,
};

pub use crate::c_slice::CByteSlice;

pub mod bit_vec;

pub mod byte_grid;

pub mod command;

pub mod connection;

pub mod packet;

pub mod pixel_grid;

/// The minimum time needed for the display to refresh the screen in ms.
pub const FRAME_PACING_MS: u32 = servicepoint::FRAME_PACING.as_millis() as u32;

mod c_slice;
