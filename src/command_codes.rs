use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u16)]
#[derive(Debug, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum CommandCode {
    Clear = 0x0002,
    Cp437Data = 0x0003,
    CharBrightness = 0x0005,
    Brightness = 0x0007,
    HardReset = 0x000b,
    FadeOut = 0x000d,
    #[deprecated]
    BitmapLegacy = 0x0010,
    BitmapLinear = 0x0012,
    BitmapLinearWin = 0x0013,
    BitmapLinearAnd = 0x0014,
    BitmapLinearOr = 0x0015,
    BitmapLinearXor = 0x0016,
}

impl CommandCode {
    pub fn from_primitive(value: u16) -> Option<Self> {
        FromPrimitive::from_u16(value)
    }

    pub fn to_primitive(&self) -> u16 {
        ToPrimitive::to_u16(self).unwrap()
    }
}

#[repr(u16)]
#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy)]
pub enum CompressionCode {
    None = 0x0,
    #[cfg(feature = "compression-gz")]
    Gz = 0x677a,
    #[cfg(feature = "compression-bz")]
    Bz = 0x627a,
    #[cfg(feature = "compression-lz")]
    Lz = 0x6c7a,
    #[cfg(feature = "compression-zs")]
    Zs = 0x7a73,
}

impl CompressionCode {
    pub fn from_primitive(value: u16) -> Option<Self> {
        FromPrimitive::from_u16(value)
    }

    pub fn to_primitive(&self) -> u16 {
        ToPrimitive::to_u16(self).unwrap()
    }
}
