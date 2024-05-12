/// A grid of bytes
#[derive(Debug, Clone)]
pub struct ByteGrid {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl ByteGrid {
    /// Creates a new byte grid with the specified dimensions.
    ///
    /// returns: ByteGrid initialized to 0.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    /// Loads a byte grid with the specified dimensions from the provided data.
    ///
    /// returns: ByteGrid that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data: Vec::from(data),
            width,
            height,
        }
    }

    /// Get the current value at the specified position
    ///
    /// returns: current byte value
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.width]
    }

    /// Sets the byte value at the specified position
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[x + y * self.width] = value;
    }

    /// Sets all bytes in the grid to the specified value
    pub fn fill(&mut self, value: u8) {
        self.data.fill(value)
    }
}

impl Into<Vec<u8>> for ByteGrid {
    fn into(self) -> Vec<u8> {
        self.data
    }
}

#[allow(unused)]
pub mod c_api
{
    use crate::{ByteGrid, PixelGrid};

    /// Creates a new `ByteGrid` instance.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    pub unsafe extern "C" fn byte_grid_new(width: usize, height: usize) -> *mut ByteGrid {
        Box::into_raw(Box::new(ByteGrid::new(width, height)))
    }

    /// Loads a `ByteGrid` with the specified dimensions from the provided data.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    pub unsafe extern "C" fn byte_grid_load(width: usize, height: usize, data: *const u8, data_length: usize) -> *mut ByteGrid {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(ByteGrid::load(width, height, data)))
    }

    /// Clones a `ByteGrid`.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    pub unsafe extern "C" fn byte_grid_clone(this: *const ByteGrid) -> *mut ByteGrid {
        Box::into_raw(Box::new((*this).clone()))
    }

    /// Deallocates a `ByteGrid`.
    ///
    /// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
    pub unsafe extern "C" fn byte_grid_dealloc(this: *mut ByteGrid) {
        _ = Box::from_raw(this);
    }

    /// Get the current value at the specified position
    pub unsafe extern "C" fn byte_grid_get(this: *const ByteGrid, x: usize, y: usize) -> u8 {
        (*this).get(x, y)
    }

    /// Sets the current value at the specified position
    pub unsafe extern "C" fn byte_grid_set(this: *mut ByteGrid, x: usize, y: usize, value: u8) {
        (*this).set(x, y, value);
    }

    /// Fills the whole `ByteGrid` with the specified value
    pub unsafe extern "C" fn byte_grid_fill(this: *mut ByteGrid, value: u8) {
        (*this).fill(value);
    }

    /// Gets the width in pixels of the `ByteGrid` instance.
    pub unsafe extern "C" fn pixel_grid_width(this: *const PixelGrid) -> usize {
        (*this).width
    }

    /// Gets the height in pixels of the `ByteGrid` instance.
    pub unsafe extern "C" fn pixel_grid_height(this: *const PixelGrid) -> usize {
        (*this).height
    }
}