use super::geometry::{COLUMNS, ROWS};

const FB_WIDTH: usize = COLUMNS * 8;
const FB_HEIGHT: usize = ROWS * 8;

pub enum Graphics {
    /// Raw is a series
    Raw(Raw),
}

/// Raw: Offset + Raw pixel content.
/// Pixels content: series of byte-sized 8 pixel
/// horizontal blocks. highest bit is the top left pixel.
pub struct Raw(pub u16, pub Vec<u8>);

/// A framebuffer holds 8bit pixel data.
/// The value of each pixel encodes the luminance,
/// unfortunatley this can only be set per block - so the average
/// across 8 pixels is used.
///
/// There are 56 segments and 20 rows, with 8x8 pixels per segment.
pub struct Framebuffer {
    data: Vec<u8>,
}

impl Framebuffer {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(FB_WIDTH * FB_HEIGHT),
        }
    }

    /// Convert to pixel data. (Not convinced this is correct...)
    pub fn into_bitmap(&self) -> Vec<u8> {
        let mut bitmap = Vec::with_capacity(COLUMNS * ROWS);
        for (i, v) in self.data.iter().enumerate() {
            let offset = i / 8;
            let pixel = i % 8;
            let shift = 7 - pixel;
            if *v > 0 {
                bitmap[offset] |= 1 << shift
            } else {
                bitmap[offset] &= !(1 << shift)
            }
        }
        bitmap
    }

    // Convert to luminance map
    // ... TODO
}
