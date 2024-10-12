use crate::cp437::Cp437LoadError::InvalidChar;
use crate::{Grid, PrimitiveGrid};

/// A grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
pub type Cp437Grid = PrimitiveGrid<u8>;

#[derive(Debug)]
pub enum Cp437LoadError {
    InvalidChar { index: usize, char: char },
}

impl Cp437Grid {
    /// Load an ASCII-only [&str] into a [Cp437Grid] of specified width.
    ///
    /// # Panics
    ///
    /// - for width == 0
    /// - on empty strings
    pub fn load_ascii(
        value: &str,
        width: usize,
        wrap: bool,
    ) -> Result<Self, Cp437LoadError> {
        assert!(width > 0);
        assert!(!value.is_empty());

        let mut chars = {
            let mut x = 0;
            let mut y = 0;

            for (index, char) in value.chars().enumerate() {
                if !char.is_ascii() {
                    return Err(InvalidChar { index, char });
                }

                let is_lf = char == '\n';
                if is_lf || (wrap && x == width) {
                    y += 1;
                    x = 0;
                    if is_lf {
                        continue;
                    }
                }

                x += 1;
            }

            Cp437Grid::new(width, y + 1)
        };

        let mut x = 0;
        let mut y = 0;
        for char in value.chars().map(move |c| c as u8) {
            let is_lf = char == b'\n';
            if is_lf || (wrap && x == width) {
                y += 1;
                x = 0;
                if is_lf {
                    continue;
                }
            }

            if wrap || x < width {
                chars.set(x, y, char);
            }
            x += 1;
        }

        Ok(chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_ascii_nowrap() {
        let chars = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']
            .map(move |c| c as u8);
        let expected = Cp437Grid::load(5, 2, &chars);

        let actual = Cp437Grid::load_ascii("Hello,\nWorld!", 5, false).unwrap();
        // comma will be removed because line is too long and wrap is off
        assert_eq!(actual, expected);
    }

    #[test]
    fn load_ascii_wrap() {
        let chars = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']
            .map(move |c| c as u8);
        let expected = Cp437Grid::load(5, 2, &chars);

        let actual = Cp437Grid::load_ascii("HelloWorld", 5, true).unwrap();
        // line break will be added
        assert_eq!(actual, expected);
    }
}