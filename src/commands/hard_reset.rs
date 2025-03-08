use crate::{
    commands::check_command_code_only, commands::TryFromPacketError,
    command_code::CommandCode, Packet, TypedCommand,
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
#[derive(Debug, Clone, PartialEq)]
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
