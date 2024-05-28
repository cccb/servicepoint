//! C API wrapper for the `servicepoint` crate.

pub use servicepoint::{
    CompressionCode, PIXEL_COUNT, PIXEL_HEIGHT, PIXEL_WIDTH, TILE_HEIGHT,
    TILE_SIZE, TILE_WIDTH,
};

/// C functions for interacting with `BitVec`s
pub mod bit_vec;

/// C functions for interacting with `ByteGrid`s
pub mod byte_grid;

/// C functions for interacting with `BitVec`s
pub mod c_slice;

/// C functions for interacting with `Command`s
pub mod command;

/// C functions for interacting with `Connection`s
pub mod connection;

/// C functions for interacting with `Packet`s
pub mod packet;

/// C functions for interacting with `PixelGrid`s
pub mod pixel_grid;

/// The minimum time needed for the display to refresh the screen in ms.
pub const FRAME_PACING_MS: u32 = servicepoint::FRAME_PACING.as_millis() as u32;

#[repr(C)]
/// Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
///
/// Usage of this type is inherently unsafe.
pub struct CByteSlice {
    /// The start address of the memory
    pub start: *mut u8,
    /// The amount of memory in bytes
    pub length: usize,
}
