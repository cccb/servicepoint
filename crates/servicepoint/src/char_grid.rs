use crate::primitive_grid::SeriesError;
use crate::{Grid, PrimitiveGrid};

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

impl From<&str> for CharGrid {
    fn from(value: &str) -> Self {
        let value = value.replace("\r\n", "\n");
        let mut lines = value
            .split('\n')
            .map(move |line| line.trim_end())
            .collect::<Vec<_>>();
        let width =
            lines.iter().fold(0, move |a, x| std::cmp::max(a, x.len()));

        while lines.last().is_some_and(move |line| line.is_empty()) {
            _ = lines.pop();
        }

        let mut grid = Self::new(width, lines.len());
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid.set(x, y, char);
            }
        }

        grid
    }
}

impl From<String> for CharGrid {
    fn from(value: String) -> Self {
        CharGrid::from(&*value)
    }
}

impl From<&CharGrid> for String {
    fn from(value: &CharGrid) -> Self {
        value
            .iter_rows()
            .map(move |chars| {
                chars
                    .collect::<String>()
                    .replace('\0', " ")
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Grid;
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

    #[test]
    fn str_to_char_grid() {
        let original = "Hello\r\nWorld!\n...\n";
        let grid = CharGrid::from(original);
        assert_eq!(3, grid.height());
        let actual = String::from(&grid);
        assert_eq!("Hello\nWorld!\n...", actual);
    }
}
