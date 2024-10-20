//! re-exported constants for use in C

use servicepoint::CompressionCode;
use std::time::Duration;

/// size of a single tile in one dimension
pub const SP_TILE_SIZE: usize = 8;

/// Display tile count in the x-direction
pub const SP_TILE_WIDTH: usize = 56;

/// Display tile count in the y-direction
pub const SP_TILE_HEIGHT: usize = 20;

/// Display width in pixels
pub const SP_PIXEL_WIDTH: usize = SP_TILE_WIDTH * SP_TILE_SIZE;

/// Display height in pixels
pub const SP_PIXEL_HEIGHT: usize = SP_TILE_HEIGHT * SP_TILE_SIZE;

/// pixel count on whole screen
pub const SP_PIXEL_COUNT: usize = SP_PIXEL_WIDTH * SP_PIXEL_HEIGHT;

/// Actual hardware limit is around 28-29ms/frame. Rounded up for less dropped packets.
pub const SP_FRAME_PACING_MS: u128 = Duration::from_millis(30).as_millis();

/// see [Brightness::MIN]
pub const SP_BRIGHTNESS_MIN: u8 = 0;
/// see [Brightness::MAX]
pub const SP_BRIGHTNESS_MAX: u8 = 11;
/// Count of possible brightness values
pub const SP_BRIGHTNESS_LEVELS: u8 = 12;

/// Specifies the kind of compression to use.
#[repr(u16)]
pub enum SPCompressionCode {
    /// no compression
    Uncompressed = 0x0,
    /// compress using flate2 with zlib header
    Zlib = 0x677a,
    /// compress using bzip2
    Bzip2 = 0x627a,
    /// compress using lzma
    Lzma = 0x6c7a,
    /// compress using Zstandard
    Zstd = 0x7a73,
}

impl TryFrom<SPCompressionCode> for CompressionCode {
    type Error = ();

    fn try_from(value: SPCompressionCode) -> Result<Self, Self::Error> {
        CompressionCode::try_from(value as u16)
    }
}
