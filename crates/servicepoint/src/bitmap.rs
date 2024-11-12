use bitvec::order::Msb0;
use bitvec::prelude::BitSlice;
use bitvec::slice::IterMut;

use crate::{BitVec, DataRef, Grid, PIXEL_HEIGHT, PIXEL_WIDTH};

/// A grid of pixels stored in packed bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    width: usize,
    height: usize,
    bit_vec: BitVec,
}

impl Bitmap {
    /// Creates a new [Bitmap] with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// - `width`: size in pixels in x-direction
    /// - `height`: size in pixels in y-direction
    ///
    /// returns: [Bitmap] initialized to all pixels off
    ///
    /// # Panics
    ///
    /// - when the width is not dividable by 8
    pub fn new(width: usize, height: usize) -> Self {
        assert_eq!(width % 8, 0);
        Self {
            width,
            height,
            bit_vec: BitVec::repeat(false, width * height),
        }
    }

    /// Creates a new pixel grid with the size of the whole screen.
    #[must_use]
    pub fn max_sized() -> Self {
        Self::new(PIXEL_WIDTH, PIXEL_HEIGHT)
    }

    /// Loads a [Bitmap] with the specified dimensions from the provided data.
    ///
    /// # Arguments
    ///
    /// - `width`: size in pixels in x-direction
    /// - `height`: size in pixels in y-direction
    ///
    /// returns: [Bitmap] that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    /// - when the width is not dividable by 8
    #[must_use]
    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width % 8, 0);
        assert_eq!(data.len(), height * width / 8);
        Self {
            width,
            height,
            bit_vec: BitVec::from_slice(data),
        }
    }

    /// Iterate over all cells in [Bitmap].
    ///
    /// Order is equivalent to the following loop:
    /// ```
    /// # use servicepoint::{Bitmap, Grid};
    /// # let grid = Bitmap::new(8,2);
    /// for y in 0..grid.height() {
    ///     for x in 0..grid.width() {
    ///         grid.get(x, y);
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &bool> {
        self.bit_vec.iter().by_refs()
    }

    /// Iterate over all cells in [Bitmap] mutably.
    ///
    /// Order is equivalent to the following loop:
    /// ```
    /// # use servicepoint::{Bitmap, Grid};
    /// # let mut grid = Bitmap::new(8,2);
    /// # let value = false;
    /// for y in 0..grid.height() {
    ///     for x in 0..grid.width() {
    ///         grid.set(x, y, value);
    ///     }
    /// }
    /// ```
    ///
    /// # Example
    /// ```
    /// # use servicepoint::{Bitmap, Grid};
    /// # let mut grid = Bitmap::new(8,2);
    /// # let value = false;
    /// for (index, mut pixel) in grid.iter_mut().enumerate() {
    ///     pixel.set(index % 2 == 0)
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<u8, Msb0> {
        self.bit_vec.iter_mut()
    }

    /// Iterate over all rows in [Bitmap] top to bottom.
    pub fn iter_rows(&self) -> IterRows {
        IterRows {
            bitmap: self,
            row: 0,
        }
    }
}

impl Grid<bool> for Bitmap {
    /// Sets the value of the specified position in the [Bitmap].
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    /// - `value`: the value to write to the cell
    ///
    /// returns: old value of the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.assert_in_bounds(x, y);
        self.bit_vec.set(x + y * self.width, value)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.assert_in_bounds(x, y);
        self.bit_vec[x + y * self.width]
    }

    /// Sets the state of all pixels in the [Bitmap].
    ///
    /// # Arguments
    ///
    /// - `this`: instance to write to
    /// - `value`: the value to set all pixels to
    fn fill(&mut self, value: bool) {
        self.bit_vec.fill(value);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl DataRef<u8> for Bitmap {
    fn data_ref_mut(&mut self) -> &mut [u8] {
        self.bit_vec.as_raw_mut_slice()
    }

    fn data_ref(&self) -> &[u8] {
        self.bit_vec.as_raw_slice()
    }
}

impl From<Bitmap> for Vec<u8> {
    /// Turns a [Bitmap] into the underlying [`Vec<u8>`].
    fn from(value: Bitmap) -> Self {
        value.bit_vec.into()
    }
}

pub struct IterRows<'t> {
    bitmap: &'t Bitmap,
    row: usize,
}

impl<'t> Iterator for IterRows<'t> {
    type Item = &'t BitSlice<u8, Msb0>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.bitmap.height {
            return None;
        }

        let start = self.row * self.bitmap.width;
        let end = start + self.bitmap.width;
        self.row += 1;
        Some(&self.bitmap.bit_vec[start..end])
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bitmap, DataRef, Grid};

    #[test]
    fn fill() {
        let mut grid = Bitmap::new(8, 2);
        assert_eq!(grid.data_ref(), [0x00, 0x00]);

        grid.fill(true);
        assert_eq!(grid.data_ref(), [0xFF, 0xFF]);

        grid.fill(false);
        assert_eq!(grid.data_ref(), [0x00, 0x00]);
    }

    #[test]
    fn get_set() {
        let mut grid = Bitmap::new(8, 2);
        assert!(!grid.get(0, 0));
        assert!(!grid.get(1, 1));

        grid.set(5, 0, true);
        grid.set(1, 1, true);
        assert_eq!(grid.data_ref(), [0x04, 0x40]);

        assert!(grid.get(5, 0));
        assert!(grid.get(1, 1));
        assert!(!grid.get(1, 0));
    }

    #[test]
    fn load() {
        let mut grid = Bitmap::new(8, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) % 2 == 0);
            }
        }

        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);

        let data: Vec<u8> = grid.into();

        let grid = Bitmap::load(8, 3, &data);
        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_x() {
        let vec = Bitmap::new(8, 2);
        vec.get(8, 1);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_y() {
        let mut vec = Bitmap::new(8, 2);
        vec.set(1, 2, false);
    }

    #[test]
    fn iter() {
        let grid = Bitmap::new(8, 2);
        assert_eq!(16, grid.iter().count())
    }

    #[test]
    fn iter_rows() {
        let grid = Bitmap::load(8, 2, &[0x04, 0x40]);
        let mut iter = grid.iter_rows();

        assert_eq!(iter.next().unwrap().count_ones(), 1);
        assert_eq!(iter.next().unwrap().count_ones(), 1);
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let mut grid = Bitmap::new(8, 2);
        for (index, mut pixel) in grid.iter_mut().enumerate() {
            pixel.set(index % 2 == 0);
        }
        assert_eq!(grid.data_ref(), [0xAA, 0xAA]);
    }

    #[test]
    fn data_ref_mut() {
        let mut grid = Bitmap::new(8, 2);
        let data = grid.data_ref_mut();
        data[1] = 0x0F;
        assert!(grid.get(7, 1));
    }
}
