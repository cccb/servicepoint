#![allow(deprecated, reason = "this implements the deprecated functionality")]
use crate::{
    command_code::CommandCode, commands::check_command_code_only,
    commands::errors::TryFromPacketError, Packet, TypedCommand,
};
use std::fmt::Debug;

/// Legacy command code, gets ignored by the real display.
///
/// Might be useful as a noop package.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// // this sends a packet that does nothing
/// connection.send_command(BitmapLegacyCommand).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated]
pub struct BitmapLegacyCommand;

impl TryFrom<Packet> for BitmapLegacyCommand {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) =
            check_command_code_only(value, CommandCode::BitmapLegacy)
        {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<BitmapLegacyCommand> for Packet {
    fn from(value: BitmapLegacyCommand) -> Self {
        Packet::from(&value)
    }
}

impl From<&BitmapLegacyCommand> for Packet {
    fn from(_: &BitmapLegacyCommand) -> Self {
        Packet::command_code_only(CommandCode::BitmapLegacy)
    }
}

impl From<BitmapLegacyCommand> for TypedCommand {
    fn from(command: BitmapLegacyCommand) -> Self {
        Self::BitmapLegacy(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{commands::tests::TestImplementsCommand, Header};

    impl TestImplementsCommand for BitmapLegacyCommand {}

    #[test]
    fn invalid_fields() {
        assert_eq!(
            BitmapLegacyCommand::try_from(Packet {
                header: Header {
                    command_code: CommandCode::BitmapLegacy.into(),
                    a: 1,
                    ..Default::default()
                },
                payload: None,
            }),
            Err(TryFromPacketError::ExtraneousHeaderValues)
        );
    }

    #[test]
    fn round_trip() {
        crate::commands::tests::round_trip(BitmapLegacyCommand.into());
    }

    #[test]
    fn round_trip_ref() {
        crate::commands::tests::round_trip_ref(&BitmapLegacyCommand.into());
    }
}
