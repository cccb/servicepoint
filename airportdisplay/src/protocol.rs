use std::convert::From;

use codepage_437::{ToCp437, CP437_WINGDINGS};

use super::{
    commands::Command,
    geometry::{Origin, Size, Window, COLUMNS, ROWS},
    graphics::{Graphics, Raw as GraphicsRaw},
    luminance::Luminance,
    text::{Buffer as TextBuffer, Raw as TextRaw, Text},
};

const CMD_TEXT_RAW: &'static [u8] = &[0x00, 0x03];
const CMD_LUM_RAW: &'static [u8] = &[0x00, 0x05];
const CMD_GFX_RAW: &'static [u8] = &[0x00, 0x12];

/// A frame holds a single encoded display command,
/// like set text at pos x, y.
pub type Frame = Vec<u8>;

/// Data is a list of commands to be sent to the display.
pub type Data = Vec<Frame>;

fn encode_u16(v: u16) -> Frame {
    vec![(v >> 8) as u8, v as u8]
}

/// Encode position data as big endian
impl From<Origin> for Frame {
    fn from(Origin(x, y): Origin) -> Self {
        [encode_u16(x), encode_u16(y)].concat()
    }
}

/// Encode size as big endian
impl From<Size> for Frame {
    fn from(Size(w, h): Size) -> Self {
        [encode_u16(w), encode_u16(h)].concat()
    }
}

/// Encode window data
impl From<Window> for Frame {
    fn from(Window(origin, size): Window) -> Self {
        [Frame::from(origin), Frame::from(size)].concat()
    }
}

/// Encode raw graphics
impl From<GraphicsRaw> for Data {
    fn from(raw: GraphicsRaw) -> Self {
        let GraphicsRaw(offset, data) = raw;
        vec![[
            CMD_GFX_RAW.into(),
            encode_u16(offset),
            encode_u16(data.len() as u16),
            encode_u16(0),
            encode_u16(0),
            data.into(),
        ]
        .concat()]
    }
}

/// Encode raw text byte command
impl From<TextRaw> for Data {
    fn from(TextRaw(origin, bytes): TextRaw) -> Data {
        let mut bytes = bytes.clone();
        bytes.truncate(COLUMNS);
        let size = Size(bytes.len() as u16, 1);
        vec![[CMD_TEXT_RAW.into(), origin.into(), size.into(), bytes].concat()]
    }
}

/// Encode luminance
impl From<Luminance> for Data {
    fn from(luminance: Luminance) -> Data {
        let Luminance(window, value) = luminance;
        vec![[CMD_LUM_RAW.into(), Vec::from(window), encode_u16(value)].concat()]
    }
}

/// Encode a text buffer as a series of commands (data).
impl From<TextBuffer> for Data {
    fn from(TextBuffer(Origin(x, y), text): TextBuffer) -> Data {
        let mut lines: Vec<&str> = text.split("\n").collect();
        lines.truncate(ROWS);

        let mut data = vec![];
        for (i, line) in lines.iter().enumerate() {
            // Convert utf8 to codepage 437
            if let Ok(bytes) = line.to_cp437(&CP437_WINGDINGS) {
                let mut bytes: Frame = bytes.into();
                bytes.truncate(COLUMNS);

                let len = bytes.len() as u16;
                let pos = Origin(x, y + i as u16);
                let size = Size(len, 1);
                data.push(
                    [
                        Frame::from(CMD_TEXT_RAW),
                        pos.into(),
                        size.into(),
                        bytes.into(),
                    ]
                    .concat(),
                );
            }
        }
        data
    }
}

/// Encode text command
impl From<Text> for Data {
    fn from(text: Text) -> Data {
        match text {
            Text::Raw(raw) => raw.into(),
            Text::Buffer(buffer) => buffer.into(),
        }
    }
}

impl From<Graphics> for Data {
    fn from(gfx: Graphics) -> Data {
        match gfx {
            Graphics::Raw(raw) => raw.into(),
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
            Command::Luminance(lum) => lum.into(),
            Command::Graphics(gfx) => gfx.into(),
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
