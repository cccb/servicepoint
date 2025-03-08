use crate::*;
use ::bitvec::{order::Msb0, prelude::BitSlice, slice::IterMut};

/// A fixed-size 2D grid of booleans.
///
/// The values are stored in packed bytes (8 values per byte) in the same order as used by the display for storing pixels.
/// This means that no conversion is necessary for sending the data to the display.
/// The downside is that the width has to be a multiple of 8.
///
/// # Examples
///
/// ```rust
/// use servicepoint::Bitmap;
/// let mut bitmap = Bitmap::new(8, 2);
///
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    width: usize,
    height: usize,
    bit_vec: BitVec,
}

impl Bitmap {
    /// Creates a new [Bitmap] with the specified dimensions.
    /// The initial state of the contained pixels is false.
    ///
    /// The width has to be a multiple of [TILE_SIZE], otherwise this function returns None.
    ///
    /// # Arguments
    ///
    /// - `width`: size in pixels in x-direction
    /// - `height`: size in pixels in y-direction
    #[must_use]
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if width % 8 != 0 {
            return None;
        }
        Some(Self::new_unchecked(width, height))
    }

    /// Creates a new pixel grid with the size of the whole screen.
    #[must_use]
    pub fn max_sized() -> Self {
        Self::new_unchecked(PIXEL_WIDTH, PIXEL_HEIGHT)
    }

    #[must_use]
    fn new_unchecked(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bit_vec: BitVec::repeat(false, width * height),
        }
    }

    /// Loads a [Bitmap] with the specified dimensions from the provided data.
    ///
    /// The data cannot be loaded on the following cases:
    /// - when the dimensions and data size do not match exactly.
    /// - when the width is not dividable by 8
    ///
    /// In those cases, an Err is returned.
    ///
    /// Otherwise, this returns a [Bitmap] that contains a copy of the provided data
    ///
    /// # Arguments
    ///
    /// - `width`: size in pixels in x-direction
    /// - `height`: size in pixels in y-direction
    pub fn load(
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Result<Self, LoadBitmapError> {
        if width % 8 != 0 {
            return Err(LoadBitmapError::InvalidWidth);
        }
        if data.len() != height * width / 8 {
            return Err(LoadBitmapError::InvalidDataSize);
        }
        Ok(Self {
            width,
            height,
            bit_vec: BitVec::from_slice(data),
        })
    }

    /// Creates a [Bitmap] with the specified width from the provided [BitVec] without copying it.
    ///
    /// The data cannot be loaded on the following cases:
    /// - when the data size is not divisible by the width (incomplete rows)
    /// - when the width is not dividable by 8
    ///
    /// In those cases, an Err is returned.
    /// Otherwise, this returns a [Bitmap] that contains the provided data.
    pub fn from_bitvec(
        width: usize,
        bit_vec: BitVec,
    ) -> Result<Self, LoadBitmapError> {
        if width % 8 != 0 {
            return Err(LoadBitmapError::InvalidWidth);
        }
        let len = bit_vec.len();
        let height = len / width;
        if len % width != 0 {
            return Err(LoadBitmapError::InvalidDataSize);
        }

        Ok(Self {
            width,
            height,
            bit_vec,
        })
    }

    /// Iterate over all cells in [Bitmap].
    ///
    /// Order is equivalent to the following loop:
    /// ```
    /// # use servicepoint::{Bitmap, Grid};
    /// # let grid = Bitmap::new(8, 2).unwrap();
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
    /// # let mut grid = Bitmap::new(8, 2).unwrap();
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
    /// # let mut grid = Bitmap::new(8, 2).unwrap();
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

impl From<Bitmap> for BitVec {
    /// Turns a [Bitmap] into the underlying [BitVec].
    fn from(value: Bitmap) -> Self {
        value.bit_vec
    }
}

impl TryFrom<&ValueGrid<bool>> for Bitmap {
    type Error = ();

    /// Converts a grid of [bool]s into a [Bitmap].
    ///
    /// Returns Err if the width of `value` is not dividable by 8
    fn try_from(value: &ValueGrid<bool>) -> Result<Self, Self::Error> {
        let mut result = Self::new(value.width(), value.height()).ok_or(())?;
        for (mut to, from) in result.iter_mut().zip(value.iter()) {
            *to = *from;
        }
        Ok(result)
    }
}

impl From<&Bitmap> for ValueGrid<bool> {
    /// Converts a [Bitmap] into a grid of [bool]s.
    fn from(value: &Bitmap) -> Self {
        let mut result = Self::new(value.width(), value.height());
        for (to, from) in result.iter_mut().zip(value.iter()) {
            *to = *from;
        }
        result
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

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum LoadBitmapError {
    #[error("The provided width is not divisible by 8.")]
    InvalidWidth,
    #[error(
        "The provided data has an incorrect size for the provided dimensions."
    )]
    InvalidDataSize,
}

#[cfg(test)]
mod tests {
    use crate::{BitVec, Bitmap, DataRef, Grid, LoadBitmapError, ValueGrid};

    #[test]
    fn fill() {
        let mut grid = Bitmap::new(8, 2).unwrap();
        assert_eq!(grid.data_ref(), [0x00, 0x00]);

        grid.fill(true);
        assert_eq!(grid.data_ref(), [0xFF, 0xFF]);

        grid.fill(false);
        assert_eq!(grid.data_ref(), [0x00, 0x00]);
    }

    #[test]
    fn get_set() {
        let mut grid = Bitmap::new(8, 2).unwrap();
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
        let mut grid = Bitmap::new(8, 3).unwrap();
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) % 2 == 0);
            }
        }

        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);

        let data: Vec<u8> = grid.into();

        let grid = Bitmap::load(8, 3, &data).unwrap();
        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_x() {
        let vec = Bitmap::new(8, 2).unwrap();
        vec.get(8, 1);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_y() {
        let mut vec = Bitmap::new(8, 2).unwrap();
        vec.set(1, 2, false);
    }

    #[test]
    fn iter() {
        let grid = Bitmap::new(8, 2).unwrap();
        assert_eq!(16, grid.iter().count())
    }

    #[test]
    fn iter_rows() {
        let grid = Bitmap::load(8, 2, &[0x04, 0x40]).unwrap();
        let mut iter = grid.iter_rows();

        assert_eq!(iter.next().unwrap().count_ones(), 1);
        assert_eq!(iter.next().unwrap().count_ones(), 1);
        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter_mut() {
        let mut grid = Bitmap::new(8, 2).unwrap();
        for (index, mut pixel) in grid.iter_mut().enumerate() {
            pixel.set(index % 2 == 0);
        }
        assert_eq!(grid.data_ref(), [0xAA, 0xAA]);
    }

    #[test]
    fn data_ref_mut() {
        let mut grid = Bitmap::new(8, 2).unwrap();
        let data = grid.data_ref_mut();
        data[1] = 0x0F;
        assert!(grid.get(7, 1));
    }

    #[test]
    fn to_bitvec() {
        let mut grid = Bitmap::new(8, 2).unwrap();
        grid.set(0, 0, true);
        let bitvec: BitVec = grid.into();
        assert_eq!(bitvec.as_raw_slice(), [0x80, 0x00]);
    }

    #[test]
    fn from_bool_grid() {
        let original = ValueGrid::load(
            8,
            1,
            &[true, false, true, false, true, false, true, false],
        ).unwrap();
        let converted = Bitmap::try_from(&original).unwrap();
        let reconverted = ValueGrid::from(&converted);
        assert_eq!(original, reconverted);
    }

    #[test]
    fn load_invalid_width() {
        let data = BitVec::repeat(false, 7*3).into_vec();
        assert_eq!(Bitmap::load(7, 3, &data), Err(LoadBitmapError::InvalidWidth))
    }

    #[test]
    fn load_invalid_size() {
        let data = BitVec::repeat(false, 8*4).into_vec();
        assert_eq!(Bitmap::load(8, 3, &data), Err(LoadBitmapError::InvalidDataSize))
    }

    #[test]
    fn from_vec_invalid_width() {
        let data = BitVec::repeat(false, 7*3);
        assert_eq!(Bitmap::from_bitvec(7, data), Err(LoadBitmapError::InvalidWidth))
    }

    #[test]
    fn from_vec_invalid_size() {
        let data = BitVec::repeat(false, 7*4);
        assert_eq!(Bitmap::from_bitvec(8, data), Err(LoadBitmapError::InvalidDataSize))
    }

    #[test]
    fn from_vec() {
        let orig = Bitmap::new(8, 3).unwrap();
        assert_eq!(Bitmap::from_bitvec(8, orig.bit_vec.clone()).unwrap(), orig);
    }

    #[test]
    fn new_invalid_width() {
        assert_eq!(Bitmap::new(7,2), None)
    }
}
