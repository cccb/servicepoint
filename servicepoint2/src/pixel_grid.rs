use crate::{BitVec, PIXEL_HEIGHT, PIXEL_WIDTH};

/// A grid of pixels stored in packed bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct PixelGrid {
    width: usize,
    height: usize,
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
        self.check_indexes(x, y);
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

    pub fn mut_data_ref(&mut self) -> &mut [u8] {
        self.bit_vec.mut_data_ref()
    }

    /// the size in x-direction in pixels
    pub fn width(&self) -> usize {
        self.width
    }

    /// the height in y-direction in pixels
    pub fn height(&self) -> usize {
        self.height
    }

    fn check_indexes(&self, x: usize, y: usize) {
        if x >= self.width {
            panic!("cannot access pixel {x}-{y} because x is outside of bounds 0..{}", self.width)
        }
        if y >= self.height {
            panic!("cannot access pixel {x}-{y} because y is outside of bounds 0..{}", self.height)
        }
    }
}

impl From<PixelGrid> for Vec<u8> {
    /// Turns a `PixelGrid` into the underlying `Vec<u8>`.
    fn from(value: PixelGrid) -> Self {
        value.bit_vec.into()
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    use crate::c_slice::CByteSlice;
    use crate::PixelGrid;

    /// Creates a new `PixelGrid` instance.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_new(
        width: usize,
        height: usize,
    ) -> *mut PixelGrid {
        Box::into_raw(Box::new(PixelGrid::new(width, height)))
    }

    /// Loads a `PixelGrid` with the specified dimensions from the provided data.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_load(
        width: usize,
        height: usize,
        data: *const u8,
        data_length: usize,
    ) -> *mut PixelGrid {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(PixelGrid::load(width, height, data)))
    }

    /// Clones a `PixelGrid`.
    /// The returned instance has to be freed with `pixel_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_clone(
        this: *const PixelGrid,
    ) -> *mut PixelGrid {
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
    pub unsafe extern "C" fn sp2_pixel_grid_get(
        this: *const PixelGrid,
        x: usize,
        y: usize,
    ) -> bool {
        (*this).get(x, y)
    }

    /// Sets the current value at the specified position
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_set(
        this: *mut PixelGrid,
        x: usize,
        y: usize,
        value: bool,
    ) {
        (*this).set(x, y, value);
    }

    /// Fills the whole `PixelGrid` with the specified value
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_fill(
        this: *mut PixelGrid,
        value: bool,
    ) {
        (*this).fill(value);
    }

    /// Gets the width in pixels of the `PixelGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_width(
        this: *const PixelGrid,
    ) -> usize {
        (*this).width
    }

    /// Gets the height in pixels of the `PixelGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_height(
        this: *const PixelGrid,
    ) -> usize {
        (*this).height
    }

    /// Gets an unsafe reference to the data of the `PixelGrid` instance.
    ///
    /// ## Safety
    ///
    /// The caller has to make sure to never access the returned memory after the `PixelGrid`
    /// instance has been consumed or manually deallocated.
    ///
    /// Reading and writing concurrently to either the original instance or the returned data will
    /// result in undefined behavior.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_pixel_grid_unsafe_data_ref(
        this: *mut PixelGrid,
    ) -> CByteSlice {
        let data = (*this).mut_data_ref();
        CByteSlice {
            start: data.as_mut_ptr_range().start,
            length: data.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PixelGrid;

    #[test]
    fn fill() {
        let mut grid = PixelGrid::new(8, 2);
        assert_eq!(grid.mut_data_ref(), [0x00, 0x00]);

        grid.fill(true);
        assert_eq!(grid.mut_data_ref(), [0xFF, 0xFF]);

        grid.fill(false);
        assert_eq!(grid.mut_data_ref(), [0x00, 0x00]);
    }

    #[test]
    fn get_set() {
        let mut grid = PixelGrid::new(8, 2);
        assert!(!grid.get(0, 0));
        assert!(!grid.get(1, 1));

        grid.set(5, 0, true);
        grid.set(1, 1, true);
        assert_eq!(grid.mut_data_ref(), [0x04, 0x40]);

        assert!(grid.get(5, 0));
        assert!(grid.get(1, 1));
        assert!(!grid.get(1, 0));
    }

    #[test]
    fn load() {
        let mut grid = PixelGrid::new(8, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) % 2 == 0);
            }
        }

        assert_eq!(grid.mut_data_ref(), [0xAA, 0x55, 0xAA]);

        let data: Vec<u8> = grid.into();

        let mut grid = PixelGrid::load(8, 3, &data);
        assert_eq!(grid.mut_data_ref(), [0xAA, 0x55, 0xAA]);
    }
}
