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
    BitmapLinearWinUncompressed = 0x0013,
    BitmapLinearAnd = 0x0014,
    BitmapLinearOr = 0x0015,
    BitmapLinearXor = 0x0016,
    #[cfg(feature = "compression_zlib")]
    BitmapLinearWinZlib = 0x0017,
    #[cfg(feature = "compression_bzip2")]
    BitmapLinearWinBzip2 = 0x0018,
    #[cfg(feature = "compression_lzma")]
    BitmapLinearWinLzma = 0x0019,
    #[cfg(feature = "compression_zstd")]
    BitmapLinearWinZstd = 0x001A,
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
        match value {
            value if value == CommandCode::Clear as u16 => {
                Ok(CommandCode::Clear)
            }
            value if value == CommandCode::Cp437Data as u16 => {
                Ok(CommandCode::Cp437Data)
            }
            value if value == CommandCode::CharBrightness as u16 => {
                Ok(CommandCode::CharBrightness)
            }
            value if value == CommandCode::Brightness as u16 => {
                Ok(CommandCode::Brightness)
            }
            value if value == CommandCode::HardReset as u16 => {
                Ok(CommandCode::HardReset)
            }
            value if value == CommandCode::FadeOut as u16 => {
                Ok(CommandCode::FadeOut)
            }
            #[allow(deprecated)]
            value if value == CommandCode::BitmapLegacy as u16 => {
                Ok(CommandCode::BitmapLegacy)
            }
            value if value == CommandCode::BitmapLinear as u16 => {
                Ok(CommandCode::BitmapLinear)
            }
            value
                if value == CommandCode::BitmapLinearWinUncompressed as u16 =>
            {
                Ok(CommandCode::BitmapLinearWinUncompressed)
            }
            value if value == CommandCode::BitmapLinearAnd as u16 => {
                Ok(CommandCode::BitmapLinearAnd)
            }
            value if value == CommandCode::BitmapLinearOr as u16 => {
                Ok(CommandCode::BitmapLinearOr)
            }
            value if value == CommandCode::BitmapLinearXor as u16 => {
                Ok(CommandCode::BitmapLinearXor)
            }
            #[cfg(feature = "compression_zstd")]
            value if value == CommandCode::BitmapLinearWinZstd as u16 => {
                Ok(CommandCode::BitmapLinearWinZstd)
            }
            #[cfg(feature = "compression_lzma")]
            value if value == CommandCode::BitmapLinearWinLzma as u16 => {
                Ok(CommandCode::BitmapLinearWinLzma)
            }
            #[cfg(feature = "compression_zlib")]
            value if value == CommandCode::BitmapLinearWinZlib as u16 => {
                Ok(CommandCode::BitmapLinearWinZlib)
            }
            #[cfg(feature = "compression_bzip2")]
            value if value == CommandCode::BitmapLinearWinBzip2 as u16 => {
                Ok(CommandCode::BitmapLinearWinBzip2)
            }
            _ => Err(()),
        }
    }
}
