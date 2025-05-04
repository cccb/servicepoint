use crate::{
    command_code::CommandCode, commands::errors::TryFromPacketError,
    BitVecCommand, BitmapCommand, BrightnessGridCommand, CharGridCommand,
    ClearCommand, Cp437GridCommand, FadeOutCommand, GlobalBrightnessCommand,
    HardResetCommand, Packet, TryIntoPacketError,
};

/// This enum contains all commands provided by the library.
/// This is useful in case you want one data type for all kinds of commands without using `dyn`.
///
/// Please look at the contained structs for documentation per command.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
#[allow(deprecated)]
pub enum TypedCommand {
    Clear(ClearCommand),
    CharGrid(CharGridCommand),
    Cp437Grid(Cp437GridCommand),
    Bitmap(BitmapCommand),
    Brightness(GlobalBrightnessCommand),
    BrightnessGrid(BrightnessGridCommand),
    BitVec(BitVecCommand),
    HardReset(HardResetCommand),
    FadeOut(FadeOutCommand),
    #[deprecated]
    BitmapLegacy(crate::BitmapLegacyCommand),
}

impl TryFrom<Packet> for TypedCommand {
    type Error = TryFromPacketError;

    /// Try to interpret the [Packet] as one containing a [`TypedCommand`]
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        Ok(match CommandCode::try_from(packet.header.command_code)? {
            CommandCode::Clear => {
                TypedCommand::Clear(crate::ClearCommand::try_from(packet)?)
            }
            CommandCode::Brightness => TypedCommand::Brightness(
                crate::GlobalBrightnessCommand::try_from(packet)?,
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

impl TryFrom<TypedCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: TypedCommand) -> Result<Self, Self::Error> {
        Ok(match value {
            TypedCommand::Clear(c) => c.into(),
            TypedCommand::CharGrid(c) => c.try_into()?,
            TypedCommand::Cp437Grid(c) => c.try_into()?,
            TypedCommand::Bitmap(c) => c.try_into()?,
            TypedCommand::Brightness(c) => c.into(),
            TypedCommand::BrightnessGrid(c) => c.try_into()?,
            TypedCommand::BitVec(c) => c.try_into()?,
            TypedCommand::HardReset(c) => c.into(),
            TypedCommand::FadeOut(c) => c.into(),
            #[allow(deprecated)]
            TypedCommand::BitmapLegacy(c) => c.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        command_code::InvalidCommandCodeError,
        commands::tests::TestImplementsCommand, Header, Packet, TypedCommand,
    };

    impl TestImplementsCommand for TypedCommand {}

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
            payload: None,
        };
        let result = TypedCommand::try_from(p);
        assert_eq!(result, Err(InvalidCommandCodeError(0xFF).into()));
    }
}
