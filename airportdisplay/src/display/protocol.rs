use std::convert::From;

use bytes::BufMut;

use super::{
    commands::{Command, Text, TextBuffer, TextRaw},
    window::Window,
    TEXT_COLUMNS, TEXT_ROWS,
};

/// A frame holds a single encoded display command,
/// like set text at pos x, y.
type Frame = Vec<u8>;

/// Data is a list of commands to be sent to the display.
type Data = Vec<Frame>;

/// Encode window data
impl From<Window> for Frame {
    fn from(Window { x, y, w, h }: Window) -> Self {
        let mut buf = vec![];
        buf.put_u16_le(x);
        buf.put_u16_le(y);
        buf.put_u16_le(w);
        buf.put_u16_le(h);
        buf
    }
}

impl From<TextRaw> for Data {
    fn from(TextRaw(window, bytes): TextRaw) -> Data {
        let mut bytes = bytes.clone();
        bytes.truncate(TEXT_COLUMNS);
        vec![[vec![0x03, 0x00], window.into(), bytes].concat()]
    }
}

impl From<TextBuffer> for Data {
    fn from(TextBuffer(window, text): TextBuffer) -> Data {
        // let Window { x, y, w, h } = window;
        let mut lines: Vec<&str> = text.split("\n").collect();
        lines.truncate(TEXT_ROWS);
        vec![vec![]]
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
            Command::Reset => vec![vec![0x08, 0x00]],
            Command::Clear => vec![vec![0x02, 0x00]],
            Command::Reboot => vec![vec![0x0b, 0x00]],
            Command::Fadeout => vec![vec![0x0d, 0x00]],
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
