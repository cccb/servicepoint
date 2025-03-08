use crate::{CharGrid, Cp437Grid};
use std::collections::HashMap;

/// Contains functions to convert between UTF-8 and Codepage 437.
///
/// See <https://en.wikipedia.org/wiki/Code_page_437#Character_set>
pub struct Cp437Converter;

/// An array of 256 elements, mapping most of the CP437 values to UTF-8 characters
///
/// Mostly follows CP437, except 0x0A, which is kept for use as line ending.
///
/// See <https://en.wikipedia.org/wiki/Code_page_437#Character_set>
///
/// Mostly copied from <https://github.com/kip93/cp437-tools>. License: GPL-3.0
#[rustfmt::skip]
const CP437_TO_UTF8: [char; 256] = [
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
static UTF8_TO_CP437: once_cell::sync::Lazy<HashMap<char, u8>> =
    once_cell::sync::Lazy::new(|| {
        let pairs = CP437_TO_UTF8
            .iter()
            .enumerate()
            .map(move |(index, char)| (*char, index as u8));
        HashMap::from_iter(pairs)
    });

impl Cp437Converter {
    const MISSING_CHAR_CP437: u8 = 0x3F; // '?'

    /// Convert the provided bytes to UTF-8.
    pub fn cp437_to_str(cp437: &[u8]) -> String {
        cp437
            .iter()
            .map(move |char| Self::cp437_to_char(*char))
            .collect()
    }

    /// Convert a single CP-437 character to UTF-8.
    pub fn cp437_to_char(cp437: u8) -> char {
        CP437_TO_UTF8[cp437 as usize]
    }

    /// Convert the provided text to CP-437 bytes.
    ///
    /// Characters that are not available are mapped to '?'.
    pub fn str_to_cp437(utf8: &str) -> Vec<u8> {
        utf8.chars().map(Self::char_to_cp437).collect()
    }

    /// Convert a single UTF-8 character to CP-437.
    pub fn char_to_cp437(utf8: char) -> u8 {
        *UTF8_TO_CP437
            .get(&utf8)
            .unwrap_or(&Self::MISSING_CHAR_CP437)
    }
}

impl From<&Cp437Grid> for CharGrid {
    fn from(value: &Cp437Grid) -> Self {
        value.map(Cp437Converter::cp437_to_char)
    }
}

impl From<Cp437Grid> for CharGrid {
    fn from(value: Cp437Grid) -> Self {
        Self::from(&value)
    }
}

impl From<&CharGrid> for Cp437Grid {
    fn from(value: &CharGrid) -> Self {
        value.map(Cp437Converter::char_to_cp437)
    }
}

impl From<CharGrid> for Cp437Grid {
    fn from(value: CharGrid) -> Self {
        Self::from(&value)
    }
}

#[cfg(test)]
mod tests_feature_cp437 {
    use super::*;

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

        let cp437 = Cp437Converter::str_to_cp437(utf8);
        let actual = Cp437Converter::cp437_to_str(&cp437);
        assert_eq!(utf8, actual)
    }

    #[test]
    fn convert_invalid() {
        assert_eq!(
            Cp437Converter::cp437_to_char(Cp437Converter::char_to_cp437('😜')),
            '?'
        );
    }

    #[test]
    fn round_trip_cp437() {
        let utf8 = CharGrid::load(2, 2, &['Ä', 'x', '\n', '$']).unwrap();
        let cp437 = Cp437Grid::from(utf8.clone());
        let actual = CharGrid::from(cp437);
        assert_eq!(actual, utf8);
    }
}
