use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u16)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum DisplayCommandCode {
    Clear = 0x0002,
    Cp437data = 0x0003,
    CharBrightness = 0x0005,
    Brightness = 0x0007,
    HardReset = 0x000b,
    FadeOut = 0x000d,
    BitmapLegacy = 0x0010,
    BitmapLinear = 0x0012,
    BitmapLinearWin = 0x0013,
    BitmapLinearAnd = 0x0014,
    BitmapLinearOr = 0x0015,
    BitmapLinearXor = 0x0016,
}

impl DisplayCommandCode {
    pub fn from_primitive(value: u16) -> Option<Self> {
        FromPrimitive::from_u16(value)
    }

    pub fn to_primitive(&self) -> u16 {
        ToPrimitive::to_u16(self).unwrap()
    }
}