use crate::{
    command_code::CommandCode,
    commands::{check_command_code_only, errors::TryFromPacketError},
    Packet, TypedCommand,
};
use std::fmt::Debug;

/// Set all pixels to the off state. Does not affect brightness.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// connection.send_command(ClearCommand).unwrap();
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// ```
pub struct ClearCommand;

impl TryFrom<Packet> for ClearCommand {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) = check_command_code_only(value, CommandCode::Clear) {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<ClearCommand> for Packet {
    fn from(value: ClearCommand) -> Self {
        Packet::from(&value)
    }
}

impl From<&ClearCommand> for Packet {
    fn from(_: &ClearCommand) -> Self {
        Packet::command_code_only(CommandCode::Clear)
    }
}

impl From<ClearCommand> for TypedCommand {
    fn from(command: ClearCommand) -> Self {
        Self::Clear(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_code::InvalidCommandCodeError;
    use crate::commands::tests::TestImplementsCommand;
    use crate::Header;

    impl TestImplementsCommand for ClearCommand {}

    #[test]
    fn round_trip() {
        crate::commands::tests::round_trip(ClearCommand.into());
    }

    #[test]
    fn round_trip_ref() {
        crate::commands::tests::round_trip_ref(&ClearCommand.into());
    }

    #[test]
    fn extraneous_header_values() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Clear.into(),
                a: 0x05,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: None,
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ));
    }

    #[test]
    fn invalid_command_code() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::HardReset.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: None,
        };
        assert_eq!(
            Err(InvalidCommandCodeError(CommandCode::HardReset.into()).into()),
            ClearCommand::try_from(p)
        );
    }
}
