/// The u16 command codes used for the `Commands`.
#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub(crate) enum CommandCode {
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

impl From<CommandCode> for u16 {
    /// returns the u16 command code corresponding to the enum value
    fn from(value: CommandCode) -> Self {
        value as u16
    }
}

impl TryFrom<u16> for CommandCode {
    type Error = ();

    /// Returns the enum value for the specified `u16` or `Error` if the code is unknown.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use CommandCode::*;

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
            _ => Err(()),
        }
    }
}
