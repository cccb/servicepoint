use CommandCode::*;

/// The codes used for the commands. See the documentation on the corresponding commands.
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
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

impl Into<u16> for CommandCode {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for CommandCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            value if value == Clear as u16 => Ok(Clear),
            value if value == Cp437Data as u16 => Ok(Cp437Data),
            value if value == CharBrightness as u16 => Ok(CharBrightness),
            value if value == Brightness as u16 => Ok(Brightness),
            value if value == HardReset as u16 => Ok(HardReset),
            value if value == FadeOut as u16 => Ok(FadeOut),
            #[allow(deprecated)]
            value if value == BitmapLegacy as u16 => Ok(BitmapLegacy),
            value if value == BitmapLinear as u16 => Ok(BitmapLinear),
            value if value == BitmapLinearWin as u16 => Ok(BitmapLinearWin),
            value if value == BitmapLinearAnd as u16 => Ok(BitmapLinearAnd),
            value if value == BitmapLinearOr as u16 => Ok(BitmapLinearOr),
            value if value == BitmapLinearXor as u16 => Ok(BitmapLinearXor),
            _ => Err(())
        }
    }
}
