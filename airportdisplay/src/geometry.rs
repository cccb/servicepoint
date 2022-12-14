pub const COLUMNS: usize = 56;
pub const ROWS: usize = 20;

/// An origin marks the top left position of the
/// data sent to the display.
#[derive(Default)]
pub struct Origin(pub u16, pub u16);

/// Size defines the width and height of a window
pub struct Size(pub u16, pub u16);

impl Default for Size {
    fn default() -> Self {
        Self(1, 1)
    }
}

/// A window
#[derive(Default)]
pub struct Window(pub Origin, pub Size);

impl Window {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Window(Origin(x, y), Size(w, h))
    }
}
