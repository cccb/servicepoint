use crate::{
    commands::check_command_code_only, commands::TryFromPacketError,
    command_code::CommandCode, Packet, TypedCommand,
};
use std::fmt::Debug;

/// Set all pixels to the off state. Does not affect brightness.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// connection.send(ClearCommand).unwrap();
#[derive(Debug, Clone, PartialEq)]
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
    fn from(_: ClearCommand) -> Self {
        Packet::command_code_only(CommandCode::Clear)
    }
}

impl From<ClearCommand> for TypedCommand {
    fn from(command: ClearCommand) -> Self {
        Self::Clear(command)
    }
}
