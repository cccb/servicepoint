use crate::{
    commands::check_command_code_only, commands::TryFromPacketError,
    command_code::CommandCode, Packet, TypedCommand,
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
/// connection.send(FadeOutCommand).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
/// ```
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
    fn from(_: FadeOutCommand) -> Self {
        Packet::command_code_only(CommandCode::FadeOut)
    }
}

impl From<FadeOutCommand> for TypedCommand {
    fn from(command: FadeOutCommand) -> Self {
        Self::FadeOut(command)
    }
}
