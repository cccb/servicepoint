
/// A window for luminance and text commands
pub struct Window {
    pub x: u16, // 0..55
    pub y: u16, // 0..19
    pub w: u16, // 1..56
    pub h: u16, // 1..20
}

impl Window {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Window {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    pub fn position(x: u16, y: u16) -> Self {
        Window {
            x: x,
            y: y,
            w: 0,
            h: 0,
        }
    }
}

/// Luminance value
pub struct Luminance(Window, u8);

/// TextRaw holds bytes and a window
pub struct TextRaw(pub Window, pub Vec<u8>);

impl TextRaw {
    pub fn new(x: u16, y: u16, bytes: Vec<u8>) -> Self {
        Self(Window::position(x,y), bytes)
    }
}

/// TextBuffer holds a window an a text buffer
/// the text will be truncated to fit into the window
pub struct TextBuffer(pub Window, pub String);

impl TextBuffer {
    pub fn at(x: u16, y: u16, text: String) -> Self {
        Self(Window::position(x,y), text) 
    }

    pub fn from(text: String) -> Self {
        Self::at(0, 0, text)
    }

}

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
