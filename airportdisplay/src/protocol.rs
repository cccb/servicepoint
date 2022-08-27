use std::convert::From;

use codepage_437::{CP437_WINGDINGS, ToCp437};

use super::{
    commands::{Command},
    text,
    geometry::{Window, Origin, Size},
    TEXT_COLUMNS, TEXT_ROWS,
};

const CMD_RAW_TEXT: &'static [u8] = &[0x00, 0x03];

/// A frame holds a single encoded display command,
/// like set text at pos x, y.
pub type Frame = Vec<u8>;

/// Data is a list of commands to be sent to the display.
pub type Data = Vec<Frame>;

/// Encode position data as big endian
impl From<Origin> for Frame {
    fn from(Origin(x, y): Origin) -> Self {
        vec![
            (x >> 8) as u8, x as u8,
            (y >> 8) as u8, y as u8,
        ]
    }
}

/// Encode size as big endian
impl From<Size> for Frame {
    fn from(Size(w, h): Size) -> Self {
        vec![
            (w >> 8) as u8, w as u8,
            (h >> 8) as u8, h as u8,
        ]
    }
}

/// Encode window data
impl From<Window> for Frame {
    fn from(Window(origin, size): Window) -> Self {
        [Frame::from(origin), Frame::from(size)].concat()
    }
}

/// Encode raw text byte command
impl From<text::Raw> for Data {
    fn from(text::Raw(origin, bytes): text::Raw) -> Data {
        let mut bytes = bytes.clone();
        bytes.truncate(TEXT_COLUMNS);
        let size = Size(bytes.len() as u16, 1);
        vec![[CMD_RAW_TEXT.into(), origin.into(), size.into(), bytes].concat()]
    }
}

/// Encode a text buffer as a series of commands (data).
impl From<text::Buffer> for Data {
    fn from(text::Buffer(Origin(x, y), text): text::Buffer) -> Data {
        let mut lines: Vec<&str> = text.split("\n").collect();
        lines.truncate(TEXT_ROWS);

        let mut data = vec![];
        for (i, line) in lines.iter().enumerate() {    
            if let Ok(bytes) = line.to_cp437(&CP437_WINGDINGS) {
                let len = bytes.len() as u16;
                let pos = Origin(x, y + i as u16);
                let size = Size(len, 1);
                data.push([
                    Frame::from(CMD_RAW_TEXT),
                    pos.into(),
                    size.into(),
                    bytes.into(),
                ].concat());
            }
        }
        data
    }
}

impl From<text::Text> for Data {
    fn from(text: text::Text) -> Data {
        match text {
            text::Text::Raw(raw) => raw.into(),
            text::Text::Buffer(buffer) => buffer.into(),
        }
    }
}

/// Encode command
impl From<Command> for Data {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Reset => vec![vec![0x00, 0x08]],
            Command::Clear => vec![vec![0x00, 0x02]],
            Command::Reboot => vec![vec![0x00, 0x0b]],
            Command::Fadeout => vec![vec![0x00, 0x0d]],
            Command::Text(text) => text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_origin_big_endian() {
        let frame: Frame = Origin(23, 42).into();
        assert_eq!(frame[0], 0);
        assert_eq!(frame[1], 23);
        assert_eq!(frame[2], 0);
        assert_eq!(frame[3], 42);

        let frame: Frame = Origin(0xabcd, 0xcdef).into();
        assert_eq!(frame[0], 0xab);
        assert_eq!(frame[1], 0xcd);
        assert_eq!(frame[2], 0xcd);
        assert_eq!(frame[3], 0xef);
    }

    #[test]
    fn encode_size_big_endian() {
        let frame: Frame = Size(23, 42).into();
        assert_eq!(frame[0], 0);
        assert_eq!(frame[1], 23);
        assert_eq!(frame[2], 0);
        assert_eq!(frame[3], 42);

        let frame: Frame = Size(0xabcd, 0xcdef).into();
        assert_eq!(frame[0], 0xab);
        assert_eq!(frame[1], 0xcd);
        assert_eq!(frame[2], 0xcd);
        assert_eq!(frame[3], 0xef);
    }

    #[test]
    fn data_from_raw_text() {
        let bytes: Vec<u8> = "text123".into();
        let len: u8 = bytes.len() as u8;
        let cmd: Command = text::Raw::from(bytes).into();
        let data: Data = cmd.into();

        println!("data: {:?}\n", data);

        assert_eq!(data[0][1], 0x03); // TEXT
        assert_eq!(data[0][7], len);
    }

    #[test]
    fn data_from_text_buffer() {
        let text: String = "hello\ndisplay!".into();
        let cmd: Command = text::Buffer::from(text).into();
        let data: Data = cmd.into();

        println!("data: {:?}\n", data);

        assert_eq!(data.len(), 2); // 2 commands
        assert_eq!(data[0][1], 0x03); // TEXT
        assert_eq!(data[0][7], 5); // len(hallo)
        assert_eq!(data[1][7], 8); // len(display!)
    }
}
