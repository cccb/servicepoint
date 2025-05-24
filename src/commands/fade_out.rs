use crate::{
    command_code::CommandCode, commands::check_command_code_only,
    commands::errors::TryFromPacketError, Packet, TypedCommand,
};
use std::fmt::Debug;

/// <div class="warning">Untested</div>
///
/// Slowly decrease brightness until off or something like that?
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// connection.send_command(FadeOutCommand).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FadeOutCommand;

impl TryFrom<Packet> for FadeOutCommand {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) = check_command_code_only(value, CommandCode::FadeOut) {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<FadeOutCommand> for Packet {
    fn from(value: FadeOutCommand) -> Self {
        Packet::from(&value)
    }
}

impl From<&FadeOutCommand> for Packet {
    fn from(_: &FadeOutCommand) -> Self {
        Packet::command_code_only(CommandCode::FadeOut)
    }
}

impl From<FadeOutCommand> for TypedCommand {
    fn from(command: FadeOutCommand) -> Self {
        Self::FadeOut(command)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        command_code::CommandCode,
        commands::{errors::TryFromPacketError, tests::TestImplementsCommand},
        FadeOutCommand, Header, Packet, TypedCommand,
    };

    impl TestImplementsCommand for FadeOutCommand {}

    #[test]
    fn round_trip() {
        crate::commands::tests::round_trip(FadeOutCommand.into());
    }

    #[test]
    fn round_trip_ref() {
        crate::commands::tests::round_trip_ref(&FadeOutCommand.into());
    }

    #[test]
    fn error_extraneous_header_fade_out() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x10,
                b: 0x00,
                c: 0x00,
                d: 0x01,
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
    fn error_unexpected_payload() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: Some(vec![5, 7]),
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::UnexpectedPayloadSize {
                expected: 0,
                actual: 2
            })
        ));
    }
}
