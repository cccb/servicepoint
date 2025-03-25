use crate::{Grid, SetValueSeriesError, TryLoadValueGridError, ValueGrid};
use std::string::FromUtf8Error;

/// A grid containing UTF-8 characters.
///
/// To send a `CharGrid` to the display, use a [`crate::CharGridCommand`].
///
/// Also see [`ValueGrid`] for the non-specialized operations and examples.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// let grid = CharGrid::from("You can\nload multiline\nstrings directly");
/// assert_eq!(grid.get_row_str(1), Some("load multiline\0\0".to_string()));
///
/// # let connection = FakeConnection;
/// let command = CharGridCommand { origin: Origin::ZERO, grid };
/// connection.send(command).unwrap()
/// ```
pub type CharGrid = ValueGrid<char>;

impl CharGrid {
    /// Loads a [`CharGrid`] with the specified width from the provided text, wrapping to as many rows as needed.
    ///
    /// The passed rows are extended with '\0' if needed.
    ///
    /// returns: [`CharGrid`] that contains a copy of the provided data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let grid = CharGrid::wrap_str(2, "abc\ndef");
    /// ```
    #[must_use]
    pub fn wrap_str(width: usize, text: &str) -> Self {
        let lines = text
            .split('\n')
            .flat_map(move |x| {
                x.chars()
                    .collect::<Vec<char>>()
                    .chunks(width)
                    .map(|c| {
                        let mut s = String::from_iter(c);
                        s.push_str(&"\0".repeat(width - s.chars().count()));
                        s
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();
        let height = lines.len();
        let mut result = Self::new(width, height);
        for (row, text_line) in lines.iter().enumerate() {
            #[allow(clippy::unwrap_used)]
            // we calculated the width before setting
            result.set_row_str(row, text_line).unwrap();
        }
        result
    }

    /// Copies a column from the grid as a String.
    ///
    /// Returns [None] if x is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let grid = CharGrid::from("ab\ncd");
    /// let col = grid.get_col_str(0).unwrap(); // "ac"
    /// ```
    #[must_use]
    pub fn get_col_str(&self, x: usize) -> Option<String> {
        Some(String::from_iter(self.get_col(x)?))
    }

    /// Copies a row from the grid as a String.
    ///
    /// Returns [None] if y is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let grid = CharGrid::from("ab\ncd");
    /// let row = grid.get_row_str(0).unwrap(); // "ab"
    /// ```
    #[must_use]
    pub fn get_row_str(&self, y: usize) -> Option<String> {
        Some(String::from_iter(self.get_row(y)?))
    }

    /// Overwrites a row in the grid with a str.
    ///
    /// Returns [`SetValueSeriesError`] if y is out of bounds or `row` is not of the correct size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let mut grid = CharGrid::from("ab\ncd");
    /// grid.set_row_str(0, "ef").unwrap();
    /// ```
    pub fn set_row_str(
        &mut self,
        y: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError> {
        self.set_row(y, value.chars().collect::<Vec<_>>().as_ref())
    }

    /// Overwrites a column in the grid with a str.
    ///
    /// Returns [`SetValueSeriesError`] if y is out of bounds or `row` is not of the correct size.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let mut grid = CharGrid::from("ab\ncd");
    /// grid.set_col_str(0, "ef").unwrap();
    /// ```
    pub fn set_col_str(
        &mut self,
        x: usize,
        value: &str,
    ) -> Result<(), SetValueSeriesError> {
        self.set_col(x, value.chars().collect::<Vec<_>>().as_ref())
    }

    /// Loads a [`CharGrid`] with the specified dimensions from the provided UTF-8 bytes.
    ///
    /// returns: [`CharGrid`] that contains the provided data, or [`FromUtf8Error`] if the data is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use servicepoint::CharGrid;
    /// let grid = CharGrid::load_utf8(2, 2, [97u8, 98, 99, 100].to_vec());
    /// ```
    pub fn load_utf8(
        width: usize,
        height: usize,
        bytes: Vec<u8>,
    ) -> Result<CharGrid, LoadUtf8Error> {
        let s: Vec<char> = String::from_utf8(bytes)?.chars().collect();
        CharGrid::load(width, height, &s).ok_or(LoadUtf8Error::TryLoadError(
            TryLoadValueGridError::InvalidDimensions,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadUtf8Error {
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    TryLoadError(#[from] TryLoadValueGridError),
}

impl From<&str> for CharGrid {
    fn from(value: &str) -> Self {
        let value = value.replace("\r\n", "\n");
        let mut lines = value.split('\n').collect::<Vec<_>>();
        let width = lines
            .iter()
            .fold(0, move |a, x| std::cmp::max(a, x.chars().count()));

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

impl From<CharGrid> for String {
    fn from(grid: CharGrid) -> Self {
        String::from(&grid)
    }
}

impl From<&CharGrid> for String {
    /// Converts a [`CharGrid`] into a [String].
    ///
    /// Rows are separated by '\n'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::CharGrid;
    /// let grid = CharGrid::from("ab\ncd");
    /// let string = String::from(grid);
    /// let grid = CharGrid::from(string);
    /// ```
    fn from(value: &CharGrid) -> Self {
        value
            .iter_rows()
            .map(String::from_iter)
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl From<&CharGrid> for Vec<u8> {
    /// Converts a [`CharGrid`] into a [`Vec<u8>`].
    ///
    /// Rows are not separated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{CharGrid, Grid};
    /// let grid = CharGrid::from("ab\ncd");
    /// let height = grid.height();
    /// let width = grid.width();
    /// let grid = CharGrid::load_utf8(width, height, grid.into());
    /// ```
    fn from(value: &CharGrid) -> Self {
        String::from_iter(value.iter()).into_bytes()
    }
}

impl From<CharGrid> for Vec<u8> {
    /// See [`From<&CharGrid>::from`].
    fn from(value: CharGrid) -> Self {
        Self::from(&value)
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
            Err(SetValueSeriesError::InvalidLength {
                expected: 2,
                actual: 3
            })
        );
        assert_eq!(grid.set_row_str(1, "ab"), Ok(()));
        assert_eq!(grid.get_row_str(1), Some(String::from("ab")));
    }

    #[test]
    fn str_to_char_grid() {
        // conversion with .to_string() covers one more line
        let original = "Hello\r\nWorld!\n...\n".to_string();

        let grid = CharGrid::from(original);
        assert_eq!(3, grid.height());
        assert_eq!("Hello\0\nWorld!\n...\0\0\0", String::from(grid));
    }

    #[test]
    fn round_trip_bytes() {
        let grid = CharGrid::from("Hello\0\nWorld!\n...\0\0\0");
        let bytes: Vec<u8> = grid.clone().into();
        let copy =
            CharGrid::load_utf8(grid.width(), grid.height(), bytes).unwrap();
        assert_eq!(grid, copy);
    }

    #[test]
    fn round_trip_string() {
        let grid = CharGrid::from("Hello\0\nWorld!\n...\0\0\0");
        let str: String = grid.clone().into();
        let copy = CharGrid::from(str);
        assert_eq!(grid, copy);
    }

    #[test]
    fn wrap_str() {
        let grid = CharGrid::wrap_str(2, "abc\ndef");
        assert_eq!(4, grid.height());
        assert_eq!("ab\nc\0\nde\nf\0", String::from(grid));
    }
}
