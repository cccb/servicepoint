use std::convert::From;

use super::{
    luminance::Luminance,
    text::{Buffer as TextBuffer, Raw as TextRaw, Text},
};

/// Display Commands
pub enum Command {
    Reset,
    Clear,
    Reboot,
    Fadeout,
    Text(Text),
    Luminance(Luminance),
}

/// Directly converty a raw text into a command which
/// can be sent to the display.
impl From<TextRaw> for Command {
    fn from(raw: TextRaw) -> Self {
        Command::Text(Text::Raw(raw))
    }
}

/// Shortcut to directly convert a text buffer into
/// a commmand which can be sent to the display.
impl From<TextBuffer> for Command {
    fn from(buffer: TextBuffer) -> Self {
        Command::Text(Text::Buffer(buffer))
    }
}

/// Shortcut to convert a luminance window
/// to a command.
impl From<Luminance> for Command {
    fn from(luminance: Luminance) -> Self {
        Command::Luminance(luminance)
    }
}
