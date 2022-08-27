use std::convert::From;

use bytes::BufMut;
use codepage_437::{CP437_WINGDINGS, ToCp437};

use super::{
    commands::{Command, Text, TextBuffer, TextRaw, Window},
    TEXT_COLUMNS, TEXT_ROWS,
};

/// A frame holds a single encoded display command,
/// like set text at pos x, y.
pub type Frame = Vec<u8>;

/// Data is a list of commands to be sent to the display.
pub type Data = Vec<Frame>;

/// Encode window data
impl From<Window> for Frame {
    fn from(Window { x, y, w, h }: Window) -> Self {
        let mut buf = vec![];
        buf.put_u16(x);
        buf.put_u16(y);
        buf.put_u16(w);
        buf.put_u16(h);
        buf
    }
}

impl From<TextRaw> for Data {
    fn from(TextRaw(position, bytes): TextRaw) -> Data {
        let mut bytes = bytes.clone();
        bytes.truncate(TEXT_COLUMNS);
        let window = Window::new(position.x, position.y, bytes.len() as u16, 1);
        vec![[vec![0x00, 0x03], window.into(), bytes].concat()]
    }
}

impl From<TextBuffer> for Data {
    fn from(TextBuffer(position, text): TextBuffer) -> Data {
        let mut lines: Vec<&str> = text.split("\n").collect();
        lines.truncate(TEXT_ROWS);

        let mut data = vec![];
        for (i, line) in lines.iter().enumerate() {    
            if let Ok(bytes) = line.to_cp437(&CP437_WINGDINGS) {
                let len = bytes.len() as u16;
                let window = Window::new(position.x, position.y + i as u16, len as u16, 1);
                data.push([
                    vec![0x00, 0x03],
                    window.into(),
                    bytes.into(),
                ].concat());
            }
        }
        data
    }
}

impl From<Text> for Data {
    fn from(text: Text) -> Data {
        match text {
            Text::Raw(raw) => raw.into(),
            Text::Buffer(buffer) => buffer.into(),
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
    use super::{Command, Data, Text, TextRaw, Window};

    #[test]
    fn frame_data_from_text_command() {
        let cmd = Command::Text(Text::Raw(TextRaw(Window::position(1, 2), "text123".into())));
        let data: Data = cmd.into();
        println!("data: {:?}", data)
    }
}
