use crate::{
    command_code::CommandCode, BitVecCommand, BitmapCommand, BrightnessCommand,
    BrightnessGridCommand, CharGridCommand, ClearCommand, Cp437GridCommand,
    FadeOutCommand, HardResetCommand, Header, LoadBitmapError, Packet,
};

/// This enum contains all commands provided by the library.
/// This is useful in case you want one data type for all kinds of commands without using `dyn`.
///
/// Please look at the contained structs for documentation per command.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum TypedCommand {
    Clear(ClearCommand),

    CharGrid(CharGridCommand),

    Cp437Grid(Cp437GridCommand),

    Bitmap(BitmapCommand),

    Brightness(BrightnessCommand),

    BrightnessGrid(BrightnessGridCommand),

    BitVec(BitVecCommand),

    HardReset(HardResetCommand),

    FadeOut(FadeOutCommand),

    #[allow(deprecated)]
    #[deprecated]
    BitmapLegacy(crate::BitmapLegacyCommand),
}

/// Err values for [TypedCommand::try_from].
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TryFromPacketError {
    /// the contained command code does not correspond to a known command
    #[error("The command code {0:?} does not correspond to a known command")]
    InvalidCommand(u16),
    /// the expected payload size was n, but size m was found
    #[error("the expected payload size was {0}, but size {1} was found")]
    UnexpectedPayloadSize(usize, usize),
    /// Header fields not needed for the command have been used.
    ///
    /// Note that these commands would usually still work on the actual display.
    #[error("Header fields not needed for the command have been used")]
    ExtraneousHeaderValues,
    /// The contained compression code is not known. This could be of disabled features.
    #[error("The compression code {0:?} does not correspond to a known compression algorithm.")]
    InvalidCompressionCode(u16),
    /// Decompression of the payload failed. This can be caused by corrupted packets.
    #[error("The decompression of the payload failed")]
    DecompressionFailed,
    /// The given brightness value is out of bounds
    #[error("The given brightness value {0} is out of bounds.")]
    InvalidBrightness(u8),
    /// Some provided text was not valid UTF-8.
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    LoadBitmapFailed(#[from] LoadBitmapError),
}

impl TryFrom<Packet> for TypedCommand {
    type Error = TryFromPacketError;

    /// Try to interpret the [Packet] as one containing a [Command]
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header: Header { command_code, .. },
            ..
        } = packet;
        let command_code = match CommandCode::try_from(command_code) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCommand(command_code));
            }
            Ok(value) => value,
        };

        Ok(match command_code {
            CommandCode::Clear => {
                TypedCommand::Clear(crate::ClearCommand::try_from(packet)?)
            }
            CommandCode::Brightness => TypedCommand::Brightness(
                crate::BrightnessCommand::try_from(packet)?,
            ),
            CommandCode::HardReset => TypedCommand::HardReset(
                crate::HardResetCommand::try_from(packet)?,
            ),
            CommandCode::FadeOut => {
                TypedCommand::FadeOut(crate::FadeOutCommand::try_from(packet)?)
            }
            CommandCode::Cp437Data => TypedCommand::Cp437Grid(
                crate::Cp437GridCommand::try_from(packet)?,
            ),
            CommandCode::CharBrightness => TypedCommand::BrightnessGrid(
                crate::BrightnessGridCommand::try_from(packet)?,
            ),
            CommandCode::Utf8Data => TypedCommand::CharGrid(
                crate::CharGridCommand::try_from(packet)?,
            ),
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => TypedCommand::BitmapLegacy(
                crate::BitmapLegacyCommand::try_from(packet)?,
            ),
            CommandCode::BitmapLinear
            | CommandCode::BitmapLinearOr
            | CommandCode::BitmapLinearAnd
            | CommandCode::BitmapLinearXor => {
                TypedCommand::BitVec(crate::BitVecCommand::try_from(packet)?)
            }
            CommandCode::BitmapLinearWinUncompressed => {
                TypedCommand::Bitmap(crate::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_zlib")]
            CommandCode::BitmapLinearWinZlib => {
                TypedCommand::Bitmap(crate::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_bzip2")]
            CommandCode::BitmapLinearWinBzip2 => {
                TypedCommand::Bitmap(crate::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_lzma")]
            CommandCode::BitmapLinearWinLzma => {
                TypedCommand::Bitmap(crate::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_zstd")]
            CommandCode::BitmapLinearWinZstd => {
                TypedCommand::Bitmap(crate::BitmapCommand::try_from(packet)?)
            }
        })
    }
}

impl From<TypedCommand> for Packet {
    fn from(command: TypedCommand) -> Self {
        match command {
            TypedCommand::Clear(c) => c.into(),
            TypedCommand::CharGrid(c) => c.into(),
            TypedCommand::Cp437Grid(c) => c.into(),
            TypedCommand::Bitmap(c) => c.into(),
            TypedCommand::Brightness(c) => c.into(),
            TypedCommand::BrightnessGrid(c) => c.into(),
            TypedCommand::BitVec(c) => c.into(),
            TypedCommand::HardReset(c) => c.into(),
            TypedCommand::FadeOut(c) => c.into(),
            #[allow(deprecated)]
            TypedCommand::BitmapLegacy(c) => c.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Header, Packet, TryFromPacketError, TypedCommand};

    #[test]
    fn error_invalid_command() {
        let p = Packet {
            header: Header {
                command_code: 0xFF,
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::InvalidCommand(0xFF))
        ))
    }
}
