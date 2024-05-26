pub use servicepoint::{
    CompressionCode, PIXEL_COUNT, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT,
    TILE_SIZE, TILE_WIDTH,
};

pub mod bit_vec;
pub mod byte_grid;
pub mod c_slice;
pub mod command;
pub mod connection;
pub mod packet;
pub mod pixel_grid;

pub const FRAME_PACING_MS: u32 = servicepoint::FRAME_PACING.as_millis() as u32;
