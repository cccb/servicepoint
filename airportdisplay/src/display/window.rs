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
