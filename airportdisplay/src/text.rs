
use std::convert::From;

use super::geometry::Origin;


/// TextRaw holds bytes and a window
pub struct Raw(pub Origin, pub Vec<u8>);

/// Convert from bytes
impl From<Vec<u8>> for Raw {
    fn from(bytes: Vec<u8>) -> Self {
        Self(Origin::default(), bytes)
    }
}


/// TextBuffer holds a multiline block of utf8 text
/// data and a origin.
pub struct Buffer(pub Origin, pub String);

impl Buffer {
    pub fn at(x: u16, y: u16, text: String) -> Self {
        Self(Origin(x, y), text) 
    }
}

/// Implement convert from trait for String
impl From<String> for Buffer {
    fn from(text: String) -> Self {
        Self(Origin::default(), text)
    }
}

/// Text Commands
pub enum Text {
    Raw(Raw),
    Buffer(Buffer),
}

impl From<Raw> for Text {
    fn from(raw: Raw) -> Self {
        Text::Raw(raw)
    }
}

impl From<Buffer> for Text {
    fn from(buffer: Buffer) -> Self {
        Text::Buffer(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_from_string() {
        let buf: Buffer = String::from("hej there!").into();
        let Buffer(Origin(x, y), text) = buf;
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(text, "hej there!");
    }
}
