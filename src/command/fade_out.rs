use crate::{
    command::check_command_code_only, command::TryFromPacketError,
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
/// # let connection = connection::Fake;
/// connection.send(command::FadeOut).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
/// ```
pub struct FadeOut;

impl TryFrom<Packet> for FadeOut {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        if let Some(e) = check_command_code_only(value, CommandCode::FadeOut) {
            Err(e)
        } else {
            Ok(Self)
        }
    }
}

impl From<FadeOut> for Packet {
    fn from(_: FadeOut) -> Self {
        Packet::command_code_only(CommandCode::FadeOut)
    }
}

impl From<FadeOut> for TypedCommand {
    fn from(command: FadeOut) -> Self {
        Self::FadeOut(command)
    }
}
