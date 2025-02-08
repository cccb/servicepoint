use std::fmt::Debug;
use std::slice::{Iter, IterMut};

use crate::*;

/// A type that can be stored in a [ValueGrid], e.g. [char], [u8].
pub trait Value: Sized + Default + Copy + Clone + Debug {}
impl<T: Sized + Default + Copy + Clone + Debug> Value for T {}

/// A 2D grid of values.
///
/// The memory layout is the one the display expects in [Command]s.
///
/// This structure can be used with any type that implements the [Value] trait.
/// You can also use the concrete type aliases provided in this crate, e.g. [CharGrid] and [ByteGrid].
#[derive(Debug, Clone, PartialEq)]
pub struct ValueGrid<T: Value> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

/// Error type for methods that change a whole column or row at once
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SetValueSeriesError {
    #[error("The index {index} was out of bounds for size {size}")]
    /// The index {index} was out of bounds for size {size}
    OutOfBounds {
        /// the index where access was tried
        index: usize,
        /// the size in that direction
        size: usize,
    },
    #[error("The provided series was expected to have a length of {expected}, but was {actual}")]
    /// The provided series was expected to have a length of {expected}, but was {actual}
    InvalidLength {
        /// actual size of the provided series
        actual: usize,
        /// expected size
        expected: usize,
    },
}

impl<T: Value> ValueGrid<T> {
    /// Creates a new [ValueGrid] with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// - width: size in x-direction
    /// - height: size in y-direction
    ///
    /// returns: [ValueGrid] initialized to default value.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    /// Loads a [ValueGrid] with the specified dimensions from the provided data.
    ///
    /// returns: [ValueGrid] that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    #[must_use]
    pub fn load(width: usize, height: usize, data: &[T]) -> Self {
        assert_eq!(
            width * height,
            data.len(),
            "dimension mismatch for data {data:?}"
        );
        Self {
            data: Vec::from(data),
            width,
            height,
        }
    }

    /// Creates a [ValueGrid] with the specified width from the provided data without copying it.
    ///
    /// returns: [ValueGrid] that contains the provided data.
    ///
    /// # Panics
    ///
    /// - when the data size is not dividable by the width.
    #[must_use]
    pub fn from_vec(width: usize, data: Vec<T>) -> Self {
        let len = data.len();
        let height = len / width;
        assert_eq!(0, len % width, "dimension mismatch - len {len} is not dividable by {width}");
        Self { data, width, height }        
    }

    /// Loads a [ValueGrid] with the specified width from the provided data, wrapping to as many rows as needed.
    ///
    /// returns: [ValueGrid] that contains a copy of the provided data or [TryLoadValueGridError].
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::ValueGrid;
    /// let grid = ValueGrid::wrap(2, &[0, 1, 2, 3, 4, 5]).unwrap();
    /// ```
    pub fn wrap(
        width: usize,
        data: &[T],
    ) -> Result<Self, TryLoadValueGridError> {
        let len = data.len();
        if len % width != 0 {
            return Err(TryLoadValueGridError::InvalidDimensions);
        }
        Ok(Self::load(width, len / width, data))
    }

    /// Loads a [ValueGrid] with the specified dimensions from the provided data.
    ///
    /// returns: [ValueGrid] that contains a copy of the provided data or [TryLoadValueGridError].
    pub fn try_load(
        width: usize,
        height: usize,
        data: Vec<T>,
    ) -> Result<Self, TryLoadValueGridError> {
        if width * height != data.len() {
            return Err(TryLoadValueGridError::InvalidDimensions);
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    /// Iterate over all cells in [ValueGrid].
    ///
    /// Order is equivalent to the following loop:
    /// ```
    /// # use servicepoint::{ByteGrid, Grid};
    /// # let grid = ByteGrid::new(2,2);
    /// for y in 0..grid.height() {
    ///     for x in 0..grid.width() {
    ///         grid.get(x, y);
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Iterate over all rows in [ValueGrid] top to bottom.
    pub fn iter_rows(&self) -> IterGridRows<T> {
        IterGridRows {
            byte_grid: self,
            row: 0,
        }
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all cells from top left to bottom right.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Get a mutable reference to the current value at the specified position.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    pub fn get_ref_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.assert_in_bounds(x, y);
        &mut self.data[x + y * self.width]
    }

    /// Get a mutable reference to the current value at the specified position if position is in bounds.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    ///
    /// returns: Reference to cell or None
    pub fn get_ref_mut_optional(
        &mut self,
        x: isize,
        y: isize,
    ) -> Option<&mut T> {
        if self.is_in_bounds(x, y) {
            Some(&mut self.data[x as usize + y as usize * self.width])
        } else {
            None
        }
    }

    /// Convert between ValueGrid types.
    ///
    /// See also [Iterator::map].
    ///
    /// # Examples
    ///
    /// Use logic written for u8s and then convert to [Brightness] values for sending in a [Command].
    /// ```
    /// # fn foo(grid: &mut ByteGrid) {}
    /// # use servicepoint::{Brightness, BrightnessGrid, ByteGrid, Command, Origin, TILE_HEIGHT, TILE_WIDTH};
    /// let mut grid: ByteGrid = ByteGrid::new(TILE_WIDTH, TILE_HEIGHT);
    /// foo(&mut grid);
    /// let grid: BrightnessGrid = grid.map(Brightness::saturating_from);
    /// let command = Command::CharBrightness(Origin::ZERO, grid);
    /// ```
    /// [Brightness]: [crate::Brightness]
    /// [Command]: [crate::Command]
    pub fn map<TConverted, F>(&self, f: F) -> ValueGrid<TConverted>
    where
        TConverted: Value,
        F: Fn(T) -> TConverted,
    {
        let data = self
            .data_ref()
            .iter()
            .map(|elem| f(*elem))
            .collect::<Vec<_>>();
        ValueGrid::load(self.width(), self.height(), &data)
    }

    /// Copies a row from the grid.
    ///
    /// Returns [None] if y is out of bounds.
    pub fn get_row(&self, y: usize) -> Option<Vec<T>> {
        self.data
            .chunks_exact(self.width())
            .nth(y)
            .map(|row| row.to_vec())
    }

    /// Copies a column from the grid.
    ///
    /// Returns [None] if x is out of bounds.
    pub fn get_col(&self, x: usize) -> Option<Vec<T>> {
        self.data
            .chunks_exact(self.width())
            .map(|row| row.get(x).copied())
            .collect()
    }

    /// Overwrites a column in the grid.
    ///
    /// Returns [Err] if x is out of bounds or `col` is not of the correct size.
    pub fn set_col(
        &mut self,
        x: usize,
        col: &[T],
    ) -> Result<(), SetValueSeriesError> {
        if col.len() != self.height() {
            return Err(SetValueSeriesError::InvalidLength {
                expected: self.height(),
                actual: col.len(),
            });
        }
        let width = self.width();
        if self
            .data
            .chunks_exact_mut(width)
            .zip(col.iter())
            .map(|(row, column_value)| {
                row.get_mut(x).map(move |cell| *cell = *column_value)
            })
            .all(|cell| cell.is_some())
        {
            Ok(())
        } else {
            Err(SetValueSeriesError::OutOfBounds {
                index: x,
                size: width,
            })
        }
    }

    /// Overwrites a row in the grid.
    ///
    /// Returns [Err] if y is out of bounds or `row` is not of the correct size.
    pub fn set_row(
        &mut self,
        y: usize,
        row: &[T],
    ) -> Result<(), SetValueSeriesError> {
        let width = self.width();
        if row.len() != width {
            return Err(SetValueSeriesError::InvalidLength {
                expected: width,
                actual: row.len(),
            });
        }

        let chunk = match self.data.chunks_exact_mut(width).nth(y) {
            Some(row) => row,
            None => {
                return Err(SetValueSeriesError::OutOfBounds {
                    size: self.height(),
                    index: y,
                })
            }
        };

        chunk.copy_from_slice(row);
        Ok(())
    }
}

/// Errors that can occur when loading a grid
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TryLoadValueGridError {
    #[error("The provided dimensions do not match with the data size")]
    /// The provided dimensions do not match with the data size
    InvalidDimensions,
}

impl<T: Value> Grid<T> for ValueGrid<T> {
    /// Sets the value of the cell at the specified position in the `ValueGrid.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    /// - `value`: the value to write to the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.assert_in_bounds(x, y);
        self.data[x + y * self.width] = value;
    }

    /// Gets the current value at the specified position.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get(&self, x: usize, y: usize) -> T {
        self.assert_in_bounds(x, y);
        self.data[x + y * self.width]
    }

    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T: Value> DataRef<T> for ValueGrid<T> {
    /// Get the underlying byte rows mutable
    fn data_ref_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Get the underlying byte rows read only
    fn data_ref(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T: Value> From<ValueGrid<T>> for Vec<T> {
    /// Turn into the underlying [`Vec<u8>`] containing the rows of bytes.
    fn from(value: ValueGrid<T>) -> Self {
        value.data
    }
}

/// An iterator iver the rows in a [ValueGrid]
pub struct IterGridRows<'t, T: Value> {
    byte_grid: &'t ValueGrid<T>,
    row: usize,
}

impl<'t, T: Value> Iterator for IterGridRows<'t, T> {
    type Item = Iter<'t, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.byte_grid.height {
            return None;
        }

        let start = self.row * self.byte_grid.width;
        let end = start + self.byte_grid.width;
        let result = self.byte_grid.data[start..end].iter();
        self.row += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        value_grid::{SetValueSeriesError, ValueGrid},
        *,
    };

    #[test]
    fn fill() {
        let mut grid = ValueGrid::<usize>::new(2, 2);
        assert_eq!(grid.data, [0x00, 0x00, 0x00, 0x00]);

        grid.fill(42);
        assert_eq!(grid.data, [42; 4]);
    }

    #[test]
    fn get_set() {
        let mut grid = ValueGrid::new(2, 2);
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
        let mut grid = ValueGrid::new(2, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) as u8);
            }
        }

        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);

        let data: Vec<u8> = grid.into();

        let grid = ValueGrid::load(2, 3, &data);
        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn mut_data_ref() {
        let mut vec = ValueGrid::new(2, 2);

        let data_ref = vec.data_ref_mut();
        data_ref.copy_from_slice(&[1, 2, 3, 4]);

        assert_eq!(vec.data, [1, 2, 3, 4]);
        assert_eq!(vec.get(1, 0), 2)
    }

    #[test]
    fn iter() {
        let mut vec = ValueGrid::new(2, 2);
        vec.set(1, 1, 5);

        let mut iter = vec.iter();
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 5);
    }

    #[test]
    fn iter_mut() {
        let mut vec = ValueGrid::new(2, 3);
        for (index, cell) in vec.iter_mut().enumerate() {
            *cell = index as u8;
        }

        assert_eq!(vec.data_ref(), [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter_rows() {
        let vec = ValueGrid::load(2, 3, &[0, 1, 1, 2, 2, 3]);
        for (y, row) in vec.iter_rows().enumerate() {
            for (x, val) in row.enumerate() {
                assert_eq!(*val, (x + y) as u8);
            }
        }
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_x() {
        let mut vec = ValueGrid::load(2, 2, &[0, 1, 2, 3]);
        vec.set(2, 1, 5);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_y() {
        let vec = ValueGrid::load(2, 2, &[0, 1, 2, 3]);
        vec.get(1, 2);
    }

    #[test]
    fn ref_mut() {
        let mut vec = ValueGrid::load(2, 2, &[0, 1, 2, 3]);

        let top_left = vec.get_ref_mut(0, 0);
        *top_left += 5;

        assert_eq!(None, vec.get_ref_mut_optional(2, 2));
        assert_eq!(Some(&mut 5), vec.get_ref_mut_optional(0, 0));
    }

    #[test]
    fn optional() {
        let mut grid = ValueGrid::load(2, 2, &[0, 1, 2, 3]);
        grid.set_optional(0, 0, 5);
        grid.set_optional(-1, 0, 8);
        grid.set_optional(0, 8, 42);
        assert_eq!(grid.data, [5, 1, 2, 3]);

        assert_eq!(grid.get_optional(0, 0), Some(5));
        assert_eq!(grid.get_optional(0, 8), None);
    }

    #[test]
    fn col() {
        let mut grid = ValueGrid::load(2, 3, &[0, 1, 2, 3, 4, 5]);
        assert_eq!(grid.get_col(0), Some(vec![0, 2, 4]));
        assert_eq!(grid.get_col(1), Some(vec![1, 3, 5]));
        assert_eq!(grid.get_col(2), None);
        assert_eq!(grid.set_col(0, &[5, 7, 9]), Ok(()));
        assert_eq!(
            grid.set_col(2, &[5, 7, 9]),
            Err(SetValueSeriesError::OutOfBounds { size: 2, index: 2 })
        );
        assert_eq!(
            grid.set_col(0, &[5, 7]),
            Err(SetValueSeriesError::InvalidLength {
                expected: 3,
                actual: 2
            })
        );
        assert_eq!(grid.get_col(0), Some(vec![5, 7, 9]));
    }

    #[test]
    fn row() {
        let mut grid = ValueGrid::load(2, 3, &[0, 1, 2, 3, 4, 5]);
        assert_eq!(grid.get_row(0), Some(vec![0, 1]));
        assert_eq!(grid.get_row(2), Some(vec![4, 5]));
        assert_eq!(grid.get_row(3), None);
        assert_eq!(grid.set_row(0, &[5, 7]), Ok(()));
        assert_eq!(grid.get_row(0), Some(vec![5, 7]));
        assert_eq!(
            grid.set_row(3, &[5, 7]),
            Err(SetValueSeriesError::OutOfBounds { size: 3, index: 3 })
        );
        assert_eq!(
            grid.set_row(2, &[5, 7, 3]),
            Err(SetValueSeriesError::InvalidLength {
                expected: 2,
                actual: 3
            })
        );
    }

    #[test]
    fn wrap() {
        let grid = ValueGrid::wrap(2, &[0, 1, 2, 3, 4, 5]).unwrap();
        assert_eq!(grid.height(), 3);

        let grid = ValueGrid::wrap(4, &[0, 1, 2, 3, 4, 5]);
        assert_eq!(grid.err(), Some(TryLoadValueGridError::InvalidDimensions));
    }
}
