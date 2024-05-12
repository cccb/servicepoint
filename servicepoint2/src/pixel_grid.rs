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

#[cfg(feature = "c-api")]
pub mod c_api
{
    use crate::PixelGrid;

    /// Creates a new `PixelGrid` instance.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_new(width: usize, height: usize) -> *mut PixelGrid {
        Box::into_raw(Box::new(PixelGrid::new(width, height)))
    }

    /// Loads a `PixelGrid` with the specified dimensions from the provided data.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_load(width: usize, height: usize, data: *const u8, data_length: usize) -> *mut PixelGrid {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(PixelGrid::load(width, height, data)))
    }

    /// Clones a `PixelGrid`.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_clone(this: *const PixelGrid) -> *mut PixelGrid {
        Box::into_raw(Box::new((*this).clone()))
    }

    /// Deallocates a `PixelGrid`.
    ///
    /// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_dealloc(this: *mut PixelGrid) {
        _ = Box::from_raw(this);
    }

    /// Get the current value at the specified position
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_get(this: *const PixelGrid, x: usize, y: usize) -> bool {
        (*this).get(x, y)
    }

    /// Sets the current value at the specified position
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_set(this: *mut PixelGrid, x: usize, y: usize, value: bool) {
        (*this).set(x, y, value);
    }

    /// Fills the whole `PixelGrid` with the specified value
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_fill(this: *mut PixelGrid, value: bool) {
        (*this).fill(value);
    }

    /// Gets the width in pixels of the `PixelGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_width(this: *const PixelGrid) -> usize {
        (*this).width
    }

    /// Gets the height in pixels of the `PixelGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_height(this: *const PixelGrid) -> usize {
        (*this).height
    }
}
