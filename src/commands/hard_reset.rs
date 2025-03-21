use crate::{
    command_code::CommandCode, commands::check_command_code_only,
    commands::errors::TryFromPacketError, Packet, TypedCommand,
};
use std::fmt::Debug;

/// Kills the udp daemon on the display, which usually results in a restart.
///
/// Please do not send this in your normal program flow.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// connection.send(HardResetCommand).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardResetCommand;

impl TryFrom<Packet> for HardResetCommand {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) = check_command_code_only(value, CommandCode::HardReset)
        {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<HardResetCommand> for Packet {
    fn from(_: HardResetCommand) -> Self {
        Packet::command_code_only(CommandCode::HardReset)
    }
}

impl From<HardResetCommand> for TypedCommand {
    fn from(command: HardResetCommand) -> Self {
        Self::HardReset(command)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::tests::{round_trip, TestImplementsCommand};
    use crate::Header;

    impl TestImplementsCommand for HardResetCommand {}

    #[test]
    fn round_trip_hard_reset() {
        round_trip(HardResetCommand.into());
    }

    #[test]
    fn error_extraneous_header() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::HardReset.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x01,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }
}
