/// The u16 command codes used for the [Command]s.
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
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
    Utf8Data = 0x0020,
    #[cfg(feature = "compression_zstd")]
    BitmapLinearWinZstd = 0x001A,
}

impl From<CommandCode> for u16 {
    /// returns the u16 command code corresponding to the enum value
    fn from(value: CommandCode) -> Self {
        value as u16
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[error("The command code {0} is not known.")]
pub struct InvalidCommandCodeError(pub u16);

impl TryFrom<u16> for CommandCode {
    type Error = InvalidCommandCodeError;

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
            value if value == CommandCode::Utf8Data as u16 => {
                Ok(CommandCode::Utf8Data)
            }
            _ => Err(InvalidCommandCodeError(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clear() {
        assert_eq!(CommandCode::try_from(0x0002), Ok(CommandCode::Clear));
        assert_eq!(u16::from(CommandCode::Clear), 0x0002);
    }

    #[test]
    fn cp437_data() {
        assert_eq!(CommandCode::try_from(0x0003), Ok(CommandCode::Cp437Data));
        assert_eq!(u16::from(CommandCode::Cp437Data), 0x0003);
    }

    #[test]
    fn char_brightness() {
        assert_eq!(
            CommandCode::try_from(0x0005),
            Ok(CommandCode::CharBrightness)
        );
        assert_eq!(u16::from(CommandCode::CharBrightness), 0x0005);
    }

    #[test]
    fn brightness() {
        assert_eq!(CommandCode::try_from(0x0007), Ok(CommandCode::Brightness));
        assert_eq!(u16::from(CommandCode::Brightness), 0x0007);
    }

    #[test]
    fn hard_reset() {
        assert_eq!(CommandCode::try_from(0x000b), Ok(CommandCode::HardReset));
        assert_eq!(u16::from(CommandCode::HardReset), 0x000b);
    }

    #[test]
    fn fade_out() {
        assert_eq!(CommandCode::try_from(0x000d), Ok(CommandCode::FadeOut));
        assert_eq!(u16::from(CommandCode::FadeOut), 0x000d);
    }

    #[test]
    #[allow(deprecated)]
    fn bitmap_legacy() {
        assert_eq!(
            CommandCode::try_from(0x0010),
            Ok(CommandCode::BitmapLegacy)
        );
        assert_eq!(u16::from(CommandCode::BitmapLegacy), 0x0010);
    }

    #[test]
    fn linear() {
        assert_eq!(
            CommandCode::try_from(0x0012),
            Ok(CommandCode::BitmapLinear)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinear), 0x0012);
    }

    #[test]
    fn linear_and() {
        assert_eq!(
            CommandCode::try_from(0x0014),
            Ok(CommandCode::BitmapLinearAnd)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearAnd), 0x0014);
    }

    #[test]
    fn linear_xor() {
        assert_eq!(
            CommandCode::try_from(0x0016),
            Ok(CommandCode::BitmapLinearXor)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearXor), 0x0016);
    }

    #[test]
    #[cfg(feature = "compression_zlib")]
    fn bitmap_win_zlib() {
        assert_eq!(
            CommandCode::try_from(0x0017),
            Ok(CommandCode::BitmapLinearWinZlib)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearWinZlib), 0x0017);
    }

    #[test]
    #[cfg(feature = "compression_bzip2")]
    fn bitmap_win_bzip2() {
        assert_eq!(
            CommandCode::try_from(0x0018),
            Ok(CommandCode::BitmapLinearWinBzip2)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearWinBzip2), 0x0018);
    }

    #[test]
    #[cfg(feature = "compression_lzma")]
    fn bitmap_win_lzma() {
        assert_eq!(
            CommandCode::try_from(0x0019),
            Ok(CommandCode::BitmapLinearWinLzma)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearWinLzma), 0x0019);
    }

    #[test]
    #[cfg(feature = "compression_zstd")]
    fn bitmap_win_zstd() {
        assert_eq!(
            CommandCode::try_from(0x001A),
            Ok(CommandCode::BitmapLinearWinZstd)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearWinZstd), 0x001A);
    }

    #[test]
    fn bitmap_win_uncompressed() {
        assert_eq!(
            CommandCode::try_from(0x0013),
            Ok(CommandCode::BitmapLinearWinUncompressed)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearWinUncompressed), 0x0013);
    }

    #[test]
    fn utf8_data() {
        assert_eq!(CommandCode::try_from(0x0020), Ok(CommandCode::Utf8Data));
        assert_eq!(u16::from(CommandCode::Utf8Data), 0x0020);
    }

    #[test]
    fn linear_or() {
        assert_eq!(
            CommandCode::try_from(0x0015),
            Ok(CommandCode::BitmapLinearOr)
        );
        assert_eq!(u16::from(CommandCode::BitmapLinearOr), 0x0015);
    }
}
