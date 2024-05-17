/// A 2D grid of bytes
#[derive(Debug, Clone, PartialEq)]
pub struct ByteGrid {
    width: usize,
    height: usize,
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

    /// Get the underlying byte rows
    pub fn mut_data_ref(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// the size in x-direction
    pub fn width(&self) -> usize {
        self.width
    }

    /// the height in y-direction
    pub fn height(&self) -> usize {
        self.height
    }
}

impl From<ByteGrid> for Vec<u8> {
    /// Turn into the underlying `Vec<u8>` containing the rows of bytes.
    fn from(value: ByteGrid) -> Self {
        value.data
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    use crate::{ByteGrid, CByteSlice};

    /// Creates a new `ByteGrid` instance.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_new(
        width: usize,
        height: usize,
    ) -> *mut ByteGrid {
        Box::into_raw(Box::new(ByteGrid::new(width, height)))
    }

    /// Loads a `ByteGrid` with the specified dimensions from the provided data.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_load(
        width: usize,
        height: usize,
        data: *const u8,
        data_length: usize,
    ) -> *mut ByteGrid {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(ByteGrid::load(width, height, data)))
    }

    /// Clones a `ByteGrid`.
    /// The returned instance has to be freed with `byte_grid_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_clone(
        this: *const ByteGrid,
    ) -> *mut ByteGrid {
        Box::into_raw(Box::new((*this).clone()))
    }

    /// Deallocates a `ByteGrid`.
    ///
    /// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_dealloc(this: *mut ByteGrid) {
        _ = Box::from_raw(this);
    }

    /// Get the current value at the specified position
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_get(
        this: *const ByteGrid,
        x: usize,
        y: usize,
    ) -> u8 {
        (*this).get(x, y)
    }

    /// Sets the current value at the specified position
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_set(
        this: *mut ByteGrid,
        x: usize,
        y: usize,
        value: u8,
    ) {
        (*this).set(x, y, value);
    }

    /// Fills the whole `ByteGrid` with the specified value
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_fill(
        this: *mut ByteGrid,
        value: u8,
    ) {
        (*this).fill(value);
    }

    /// Gets the width in pixels of the `ByteGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_width(
        this: *const ByteGrid,
    ) -> usize {
        (*this).width
    }

    /// Gets the height in pixels of the `ByteGrid` instance.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_height(
        this: *const ByteGrid,
    ) -> usize {
        (*this).height
    }

    /// Gets an unsafe reference to the data of the `ByteGrid` instance.
    ///
    /// ## Safety
    ///
    /// The caller has to make sure to never access the returned memory after the `ByteGrid`
    /// instance has been consumed or manually deallocated.
    ///
    /// Reading and writing concurrently to either the original instance or the returned data will
    /// result in undefined behavior.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_byte_grid_unsafe_data_ref(
        this: *mut ByteGrid,
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
    use crate::ByteGrid;

    #[test]
    fn fill() {
        let mut grid = ByteGrid::new(2, 2);
        assert_eq!(grid.data, [0x00, 0x00, 0x00, 0x00]);

        grid.fill(42);
        assert_eq!(grid.data, [42; 4]);
    }

    #[test]
    fn get_set() {
        let mut grid = ByteGrid::new(2, 2);
        assert_eq!(grid.get(0, 0), 0);
        assert_eq!(grid.get(1, 1), 0);

        grid.set(0, 0, 42);
        grid.set(1, 0, 23);
        assert_eq!(grid.data, [42, 23, 0, 0]);

        assert_eq!(grid.get(0, 0), 42);
        assert_eq!(grid.get(1, 0), 23);
        assert_eq!(grid.get(1, 1), 0);
    }

    #[test]
    fn load() {
        let mut grid = ByteGrid::new(2, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) as u8);
            }
        }


        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);

        let data: Vec<u8> = grid.into();

        let grid = ByteGrid::load(2, 3, &*data);
        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn mut_data_ref() {
        let mut vec = ByteGrid::new(2, 2);

        let data_ref = vec.mut_data_ref();
        data_ref.copy_from_slice(&[1, 2, 3, 4]);

        assert_eq!(vec.data, [1, 2, 3, 4]);
        assert_eq!(vec.get(1, 0), 2)
    }
}