use super::window::Window;

/// Luminance value
pub struct Luminance(Window, u8);

/// TextRaw holds bytes and a window
pub struct TextRaw(pub Window, pub Vec<u8>);

/// TextBuffer holds a window an a text buffer
/// the text will be truncated to fit into the window
pub struct TextBuffer(pub Window, pub String);

/// Text Commands
pub enum Text {
    Raw(TextRaw),
    Buffer(TextBuffer),
}

/// Display Commands
pub enum Command {
    Reset,
    Clear,
    Reboot,
    Fadeout,
    Text(Text),
}
