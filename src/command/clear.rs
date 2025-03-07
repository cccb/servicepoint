use crate::{
    command::check_command_code_only, command::TryFromPacketError,
    command_code::CommandCode, Packet, TypedCommand,
};
use std::fmt::Debug;

/// Set all pixels to the off state. Does not affect brightness.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{connection, Command, Connection, command};
/// # let connection = connection::Fake;
/// connection.send(command::Clear).unwrap();
#[derive(Debug, Clone, PartialEq)]
/// ```
pub struct Clear;

impl TryFrom<Packet> for Clear {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) = check_command_code_only(value, CommandCode::Clear) {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<Clear> for Packet {
    fn from(_: Clear) -> Self {
        Packet::command_code_only(CommandCode::Clear)
    }
}

impl From<Clear> for TypedCommand {
    fn from(command: Clear) -> Self {
        Self::Clear(command)
    }
}
