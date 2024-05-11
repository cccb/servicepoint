use crate::{BitVec, PIXEL_HEIGHT, PIXEL_WIDTH};

/// A grid of pixels stored in packed bytes.
#[derive(Debug, Clone)]
pub struct PixelGrid {
    /// the width in pixels
    pub width: usize,
    /// the height in pixels
    pub height: usize,
    bit_vec: BitVec,
}

impl PixelGrid {

    /// Creates a new pixel grid with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width`: size in pixels in x-direction
    /// * `height`: size in pixels in y-direction
    ///
    /// returns: PixelGrid initialized to all pixels off
    ///
    /// # Panics
    ///
    /// - when the width is not dividable by 8
    pub fn new(width: usize, height: usize) -> Self {
        assert_eq!(width % 8, 0);
        Self {
            width,
            height,
            bit_vec: BitVec::new(width * height),
        }
    }

    /// Creates a new pixel grid with the size of the whole screen.
    pub fn max_sized() -> Self {
        Self::new(PIXEL_WIDTH as usize, PIXEL_HEIGHT as usize)
    }

    /// Loads a pixel grid with the specified dimensions from the provided data.
    ///
    /// # Arguments
    ///
    /// * `width`: size in pixels in x-direction
    /// * `height`: size in pixels in y-direction
    ///
    /// returns: PixelGrid that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    /// - when the width is not dividable by 8
    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width % 8, 0);
        assert_eq!(data.len(), height * width / 8);
        Self {
            width,
            height,
            bit_vec: BitVec::from(data),
        }
    }

    /// Sets the byte value at the specified position
    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        self.bit_vec.set(x + y * self.width, value)
    }

    /// Get the current value at the specified position
    ///
    /// returns: current pixel value
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bit_vec.get(x + y * self.width)
    }

    /// Sets all pixels in the grid to the specified value
    pub fn fill(&mut self, value: bool) {
        self.bit_vec.fill(value);
    }
}

impl Into<Vec<u8>> for PixelGrid {
    fn into(self) -> Vec<u8> {
        self.bit_vec.into()
    }
}
