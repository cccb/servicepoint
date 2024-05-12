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
