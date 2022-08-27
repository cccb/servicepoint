use std::convert::From;

use super::text;

/// Display Commands
pub enum Command {
    Reset,
    Clear,
    Reboot,
    Fadeout,
    Text(text::Text),
}

/// Directly converty a raw text into a command which
/// can be sent to the display.
impl From<text::Raw> for Command {
    fn from(raw: text::Raw) -> Self {
        Command::Text(text::Text::Raw(raw))
    }
}

/// Shortcut to directly convert a text buffer into
/// a commmand which can be sent to the display.
impl From<text::Buffer> for Command {
    fn from(buffer: text::Buffer) -> Self {
        Command::Text(text::Text::Buffer(buffer))
    }
}
