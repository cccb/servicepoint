use crate::grid::RefGrid;
use crate::{DataRef, Grid};

/// A 2D grid of bytes
#[derive(Debug, Clone, PartialEq)]
pub struct ByteGrid {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ByteGrid {
    /// Creates a new `ByteGrid` with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// - width: size in x-direction
    /// - height: size in y-direction
    ///
    /// returns: `ByteGrid` initialized to 0.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    /// Loads a `ByteGrid` with the specified dimensions from the provided data.
    ///
    /// returns: `ByteGrid` that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    #[must_use]
    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data: Vec::from(data),
            width,
            height,
        }
    }

    fn check_indexes(&self, x: usize, y: usize) {
        assert!(
            x < self.width,
            "cannot access byte {x}-{y} because x is outside of bounds 0..{}",
            self.width
        );
        assert!(
            y < self.height,
            "cannot access byte {x}-{y} because y is outside of bounds 0..{}",
            self.height
        );
    }
}

impl Grid<u8> for ByteGrid {
    /// Sets the value of the cell at the specified position in the `ByteGrid.
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell
    /// * `value`: the value to write to the cell
    ///
    /// returns: old value of the cell.
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: u8) -> u8 {
        self.check_indexes(x, y);
        let pos = &mut self.data[x + y * self.width];
        let old_val = *pos;
        *pos = value;
        old_val
    }

    /// Gets the current value at the specified position.
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get(&self, x: usize, y: usize) -> u8 {
        self.check_indexes(x, y);
        self.data[x + y * self.width]
    }

    fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl DataRef for ByteGrid {
    /// Get the underlying byte rows mutable
    fn data_ref_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Get the underlying byte rows read only
    fn data_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl From<ByteGrid> for Vec<u8> {
    /// Turn into the underlying `Vec<u8>` containing the rows of bytes.
    fn from(value: ByteGrid) -> Self {
        value.data
    }
}

impl RefGrid<u8> for ByteGrid {
    fn get_ref(&self, x: usize, y: usize) -> &u8 {
        self.check_indexes(x, y);
        &self.data[x + y * self.width]
    }

    fn get_ref_optional(&self, x: isize, y: isize) -> Option<&u8> {
        if self.is_in_bounds(x, y) {
            Some(&self.data[x as usize + y as usize * self.width])
        } else {
            None
        }
    }

    fn get_ref_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        self.check_indexes(x, y);
        &mut self.data[x + y * self.width]
    }

    fn get_ref_mut_optional(&mut self, x: isize, y: isize) -> Option<&mut u8> {
        if self.is_in_bounds(x, y) {
            Some(&mut self.data[x as usize + y as usize * self.width])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ByteGrid, DataRef, Grid};

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

        let grid = ByteGrid::load(2, 3, &data);
        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn mut_data_ref() {
        let mut vec = ByteGrid::new(2, 2);

        let data_ref = vec.data_ref_mut();
        data_ref.copy_from_slice(&[1, 2, 3, 4]);

        assert_eq!(vec.data, [1, 2, 3, 4]);
        assert_eq!(vec.get(1, 0), 2)
    }
}
