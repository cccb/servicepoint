use crate::SetValueSeriesError;

/// A two-dimensional readonly grid of `T`
pub trait Grid<T> {
    /// Get the current value at the specified position
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    #[must_use]
    fn get(&self, x: usize, y: usize) -> T {
        #[allow(clippy::panic, reason = "This is the version that panics - _optional does not")]
        self.get_optional(x, y)
            .unwrap_or_else(|| panic!("Cannot access index ({x}, {y}) because it is out of bounds for a grid of dimension {}x{}", self.width(), self.height()))
    }

    /// Get the current value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: Value at position or None
    #[must_use]
    fn get_optional(&self, x: usize, y: usize) -> Option<T>;

    /// the size in x-direction
    #[must_use]
    fn width(&self) -> usize;

    /// the height in y-direction
    #[must_use]
    fn height(&self) -> usize;

    /// Checks whether the specified signed position is in grid bounds
    #[must_use]
    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    /// Asserts that the specified unsigned position is in grid bounds.
    ///
    /// # Panics
    ///
    /// When the specified position is out of bounds for this grid.
    fn assert_in_bounds(&self, x: usize, y: usize) {
        let width = self.width();
        assert!(x < width, "cannot access index [{x}, {y}] because x is outside of bounds [0..{width})");
        let height = self.height();
        assert!(y < height, "cannot access index [{x}, {y}] because y is outside of bounds [0..{height})");
    }

    /// Copies a row from the grid.
    ///
    /// Returns [None] if y is out of bounds.
    #[must_use]
    fn get_row(&self, y: usize) -> Option<Vec<T>> {
        if y >= self.height() {
            return None;
        }

        let width = self.width();
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            row.push(self.get(x, y));
        }
        Some(row)
    }

    /// Copies a column from the grid.
    ///
    /// Returns [None] if x is out of bounds.
    #[must_use]
    fn get_col(&self, x: usize) -> Option<Vec<T>> {
        if x >= self.width() {
            return None;
        }

        let height = self.height();
        let mut col = Vec::with_capacity(height);
        for y in 0..height {
            col.push(self.get(x, y));
        }
        Some(col)
    }
}

/// A two-dimensional mutable grid of `T`
pub trait GridMut<T: Clone>: Grid<T> {
    /// Sets the value at the specified position
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: T) {
        #[allow(
            clippy::expect_used,
            reason = "This is the version that panics - _optional does not"
        )]
        let worked = self.set_optional(x, y, value);
        assert!(worked, "Cannot access index ({x}, {y}) because it is out of bounds for a grid of dimension {}x{}", self.width(), self.height());
    }

    /// Sets the value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: true if the value has been set
    #[must_use]
    fn set_optional(&mut self, x: usize, y: usize, value: T) -> bool;

    /// Sets all cells in the grid to the specified value
    fn fill(&mut self, value: T);

    /// Fills the grid with the values from the provided grid.
    ///
    /// The grids have to match in size exactly.
    ///
    /// For 1D slices the equivalent would be `*slice = other_slice`.
    fn deref_assign<O: Grid<T>>(&mut self, other: &O) {
        let width = self.width();
        let height = self.height();
        assert_eq!(
            width,
            other.width(),
            "Cannot assign grid of width {} to a window of width {}",
            other.width(),
            self.width()
        );
        assert_eq!(
            height,
            other.height(),
            "Cannot assign grid of height {} to a height of width {}",
            other.height(),
            self.height()
        );
        for y in 0..height {
            for x in 0..width {
                self.set(x, y, other.get(x, y));
            }
        }
    }

    /// Overwrites a column in the grid.
    ///
    /// Returns [Err] if x is out of bounds or `col` is not of the correct size.
    fn set_col(
        &mut self,
        x: usize,
        col: &[T],
    ) -> Result<(), SetValueSeriesError> {
        let height = self.height();
        if col.len() != height {
            return Err(SetValueSeriesError::InvalidLength {
                expected: height,
                actual: col.len(),
            });
        }

        if x >= self.width() {
            return Err(SetValueSeriesError::OutOfBounds {
                size: self.width(),
                index: x,
            });
        }

        for (y, item) in col.iter().enumerate().take(height) {
            self.set(x, y, item.clone());
        }

        Ok(())
    }

    /// Overwrites a row in the grid.
    ///
    /// Returns [Err] if y is out of bounds or `row` is not of the correct size.
    fn set_row(
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

        if y >= self.height() {
            return Err(SetValueSeriesError::OutOfBounds {
                size: self.height(),
                index: y,
            });
        }

        for (x, item) in row.iter().enumerate().take(width) {
            self.set(x, y, item.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{DataRef, Grid, GridMut, SetValueSeriesError, ValueGrid};

    #[test]
    fn optional() {
        let mut grid = ValueGrid::load(2, 2, &[0, 1, 2, 3]).unwrap();
        assert!(grid.set_optional(0, 0, 5));
        assert!(!grid.set_optional(0, 8, 42));
        assert_eq!(grid.data_ref(), [5, 1, 2, 3]);

        assert_eq!(grid.get_optional(0, 0), Some(5));
        assert_eq!(grid.get_optional(0, 8), None);
    }

    #[test]
    fn col() {
        let mut grid = ValueGrid::load(2, 3, &[0, 1, 2, 3, 4, 5]).unwrap();
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
        let mut grid = ValueGrid::load(2, 3, &[0, 1, 2, 3, 4, 5]).unwrap();
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
}
