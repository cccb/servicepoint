use crate::{Grid, GridMut, SetValueSeriesError};

/// Extension methods for any [`Grid<char>`]
pub trait CharGridExt {
    /// Copies a column from the grid as a String.
    ///
    /// Returns [None] if x is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::{CharGrid, CharGridExt};
    /// let grid = CharGrid::from("ab\ncd");
    /// let col = grid.get_col_str(0).unwrap(); // "ac"
    /// ```
    #[must_use]
    fn get_col_str(&self, x: usize) -> Option<String>;

    /// Copies a row from the grid as a String.
    ///
    /// Returns [None] if y is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::{CharGrid, CharGridExt};
    /// let grid = CharGrid::from("ab\ncd");
    /// let row = grid.get_row_str(0).unwrap(); // "ab"
    /// ```
    #[must_use]
    fn get_row_str(&self, y: usize) -> Option<String>;
}

/// Extension methods for any [`GridMut<char>`].
pub trait CharGridMutExt {
    /// Overwrites a row in the grid with a str.
    ///
    /// Returns [`SetValueSeriesError`] if y is out of bounds or `row` is not of the correct size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::{CharGrid, CharGridMutExt};
    /// let mut grid = CharGrid::from("ab\ncd");
    /// grid.set_row_str(0, "ef").unwrap();
    /// ```
    fn set_row_str(
        &mut self,
        y: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError>;

    /// Overwrites a column in the grid with a str.
    ///
    /// Returns [`SetValueSeriesError`] if y is out of bounds or `row` is not of the correct size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::{CharGrid, CharGridMutExt};
    /// let mut grid = CharGrid::from("ab\ncd");
    /// grid.set_col_str(0, "ef").unwrap();
    /// ```
    fn set_col_str(
        &mut self,
        x: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError>;
}

impl<G: Grid<char>> CharGridExt for G {
    #[must_use]
    fn get_col_str(&self, x: usize) -> Option<String> {
        Some(String::from_iter(self.get_col(x)?))
    }

    #[must_use]
    fn get_row_str(&self, y: usize) -> Option<String> {
        Some(String::from_iter(self.get_row(y)?))
    }
}

impl<G: GridMut<char>> CharGridMutExt for G {
    fn set_row_str(
        &mut self,
        y: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError> {
        let width = self.width();

        let len = value.len();
        if len > width {
            return Err(SetValueSeriesError::InvalidLength {
                actual: len,
                expected: width,
            });
        }

        let height = self.height();
        if y >= height {
            return Err(SetValueSeriesError::OutOfBounds {
                index: y,
                size: height,
            });
        }

        let chars = value.chars().take(width);
        for (x, c) in chars.enumerate() {
            self.set(x, y, c);
        }

        Ok(())
    }

    fn set_col_str(
        &mut self,
        x: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError> {
        let height = self.height();

        let len = value.len();
        if len > height {
            return Err(SetValueSeriesError::InvalidLength {
                actual: len,
                expected: height,
            });
        }

        let width = self.width();
        if x >= width {
            return Err(SetValueSeriesError::OutOfBounds {
                index: x,
                size: width,
            });
        }

        let chars = value.chars().take(height);
        for (y, c) in chars.enumerate() {
            self.set(x, y, c);
        }

        Ok(())
    }
}
