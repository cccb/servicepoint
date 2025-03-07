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
/// # let connection = connections::Fake;
/// connection.send(commands::HardReset).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
/// ```
pub struct HardReset;

impl TryFrom<Packet> for HardReset {
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

impl From<HardReset> for Packet {
    fn from(_: HardReset) -> Self {
        Packet::command_code_only(CommandCode::HardReset)
    }
}

impl From<HardReset> for TypedCommand {
    fn from(command: HardReset) -> Self {
        Self::HardReset(command)
    }
}
