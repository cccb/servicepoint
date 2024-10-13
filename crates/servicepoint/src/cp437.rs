//! Conversion between UTF-8 and CP-437.
//!
//! Most of the functionality is only available with feature "cp437" enabled.

use crate::{Grid, PrimitiveGrid};
use std::collections::HashMap;

/// A grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
pub type Cp437Grid = PrimitiveGrid<u8>;

/// A grid containing UTF-8 characters.
pub type CharGrid = PrimitiveGrid<char>;

/// Errors that can occur when loading CP-437.
#[derive(Debug, PartialEq)]
pub enum Cp437LoadError {
    /// Invalid character in input prevented loading
    InvalidChar {
        /// invalid character is at this position in input
        index: usize,
        /// the invalid character
        char: char,
    },
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
                    return Err(Cp437LoadError::InvalidChar { index, char });
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

#[allow(unused)] // depends on features
pub use feature_cp437::*;

#[cfg(feature = "cp437")]
mod feature_cp437 {
    use super::*;

    /// An array of 256 elements, mapping most of the CP437 values to UTF-8 characters
    ///
    /// Mostly follows CP437, except 0x0A, which is kept for use as line ending.
    ///
    /// See <https://en.wikipedia.org/wiki/Code_page_437#Character_set>
    ///
    /// Mostly copied from https://github.com/kip93/cp437-tools. License: GPL-3.0
    #[rustfmt::skip]
    pub const CP437_TO_UTF8: [char; 256] = [
        /* 0X */ '\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '\n', '♂', '♀', '♪', '♫', '☼',
        /* 1X */ '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔',  '▲', '▼',
        /* 2X */ ' ', '!', '"', '#', '$', '%', '&', '\'','(', ')', '*', '+', ',', '-', '.', '/',
        /* 3X */ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',
        /* 4X */ '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
        /* 5X */ 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\',']', '^', '_',
        /* 6X */ '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        /* 7X */ 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂',
        /* 8X */ 'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å',
        /* 9X */ 'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ',
        /* AX */ 'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»',
        /* BX */ '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',
        /* CX */ '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧',
        /* DX */ '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        /* EX */ 'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩',
        /* FX */ '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', ' ',
    ];

    const UTF8_TO_CP437: once_cell::sync::Lazy<HashMap<char, u8>> =
        once_cell::sync::Lazy::new(|| {
            let pairs = CP437_TO_UTF8
                .iter()
                .enumerate()
                .map(move |(index, char)| (*char, index as u8));
            HashMap::from_iter(pairs)
        });

    const MISSING_CHAR_CP437: u8 = 0x3F;

    impl From<&Cp437Grid> for CharGrid {
        fn from(value: &Cp437Grid) -> Self {
            value.convert(move |cp437| cp437_to_char(*cp437))
        }
    }

    impl From<&CharGrid> for Cp437Grid {
        fn from(value: &CharGrid) -> Self {
            value.convert(move |char| char_to_cp437(*char))
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

    /// Convert the provided bytes to UTF-8.
    pub fn cp437_to_str(cp437: &[u8]) -> String {
        cp437.iter().map(move |char| cp437_to_char(*char)).collect()
    }

    /// Convert a single CP-437 character to UTF-8.
    pub fn cp437_to_char(cp437: u8) -> char {
        CP437_TO_UTF8[cp437 as usize]
    }

    /// Convert the provided text to CP-437 bytes.
    ///
    /// Characters that are not available are mapped to '?'.
    pub fn str_to_cp437(utf8: &str) -> Vec<u8> {
        utf8.chars().map(char_to_cp437).collect()
    }

    /// Convert a single UTF-8 character to CP-437.
    pub fn char_to_cp437(utf8: char) -> u8 {
        *UTF8_TO_CP437.get(&utf8).unwrap_or(&MISSING_CHAR_CP437)
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

    #[test]
    fn load_ascii_invalid() {
        assert_eq!(
            Err(Cp437LoadError::InvalidChar {
                char: '🥶',
                index: 2
            }),
            Cp437Grid::load_ascii("?#🥶42", 3, false)
        );
    }
}

#[cfg(test)]
#[cfg(feature = "cp437")]
mod tests_feature_cp437 {
    use super::*;

    #[test]
    fn round_trip_cp437() {
        let utf8 = CharGrid::load(2, 2, &['Ä', 'x', '\n', '$']);
        let cp437 = Cp437Grid::from(&utf8);
        let actual = CharGrid::from(&cp437);
        assert_eq!(actual, utf8);
    }

    #[test]
    fn convert_str() {
        // test text from https://int10h.org/oldschool-pc-fonts/fontlist/font?ibm_bios
        let utf8 = r#"A quick brown fox jumps over the lazy dog.
        0123456789 ¿?¡!`'"., <>()[]{} &@%*^#$\/

        * Wieniläinen sioux'ta puhuva ökyzombie diggaa Åsan roquefort-tacoja.
        * Ça me fait peur de fêter noël là, sur cette île bizarroïde où une mère et sa môme essaient de me tuer avec un gâteau à la cigüe brûlé.
        * Zwölf Boxkämpfer jagten Eva quer über den Sylter Deich.
        * El pingüino Wenceslao hizo kilómetros bajo exhaustiva lluvia y frío, añoraba a su querido cachorro.

        ┌─┬─┐ ╔═╦═╗ ╒═╤═╕ ╓─╥─╖
        │ │ │ ║ ║ ║ │ │ │ ║ ║ ║
        ├─┼─┤ ╠═╬═╣ ╞═╪═╡ ╟─╫─╢
        └─┴─┘ ╚═╩═╝ ╘═╧═╛ ╙─╨─╜

        ░░░░░ ▐▀█▀▌ .·∙•○°○•∙·.
        ▒▒▒▒▒ ▐ █ ▌ ☺☻ ♥♦♣♠ ♪♫☼
        ▓▓▓▓▓ ▐▀█▀▌  $ ¢ £ ¥ ₧
        █████ ▐▄█▄▌ ◄►▲▼ ←→↑↓↕↨

        ⌠
        │dx ≡ Σ √x²ⁿ·δx
        ⌡"#;

        let cp437 = str_to_cp437(utf8);
        let actual = cp437_to_str(&*cp437);
        assert_eq!(utf8, actual)
    }

    #[test]
    fn convert_invalid() {
        assert_eq!(cp437_to_char(char_to_cp437('😜')), '?');
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
