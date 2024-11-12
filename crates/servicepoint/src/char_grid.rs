use crate::primitive_grid::SeriesError;
use crate::PrimitiveGrid;

/// A grid containing UTF-8 characters.
pub type CharGrid = PrimitiveGrid<char>;

impl CharGrid {
    /// Copies a column from the grid as a String.
    ///
    /// Returns [None] if x is out of bounds.
    pub fn get_col_str(&self, x: usize) -> Option<String> {
        Some(String::from_iter(self.get_col(x)?))
    }

    /// Copies a row from the grid as a String.
    ///
    /// Returns [None] if y is out of bounds.
    pub fn get_row_str(&self, y: usize) -> Option<String> {
        Some(String::from_iter(self.get_row(y)?))
    }

    /// Overwrites a row in the grid with a str.
    ///
    /// Returns [SeriesError] if y is out of bounds or `row` is not of the correct size.
    pub fn set_row_str(
        &mut self,
        y: usize,
        value: &str,
    ) -> Result<(), SeriesError> {
        self.set_row(y, value.chars().collect::<Vec<_>>().as_ref())
    }

    /// Overwrites a column in the grid with a str.
    ///
    /// Returns [SeriesError] if y is out of bounds or `row` is not of the correct size.
    pub fn set_col_str(
        &mut self,
        x: usize,
        value: &str,
    ) -> Result<(), SeriesError> {
        self.set_col(x, value.chars().collect::<Vec<_>>().as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn col_str() {
        let mut grid = CharGrid::new(2, 3);
        assert_eq!(grid.get_col_str(2), None);
        assert_eq!(grid.get_col_str(1), Some(String::from("\0\0\0")));
        assert_eq!(grid.set_col_str(1, "abc"), Ok(()));
        assert_eq!(grid.get_col_str(1), Some(String::from("abc")));
    }

    #[test]
    fn row_str() {
        let mut grid = CharGrid::new(2, 3);
        assert_eq!(grid.get_row_str(3), None);
        assert_eq!(grid.get_row_str(1), Some(String::from("\0\0")));
        assert_eq!(
            grid.set_row_str(1, "abc"),
            Err(SeriesError::InvalidLength {
                expected: 2,
                actual: 3
            })
        );
        assert_eq!(grid.set_row_str(1, "ab"), Ok(()));
        assert_eq!(grid.get_row_str(1), Some(String::from("ab")));
    }
}
