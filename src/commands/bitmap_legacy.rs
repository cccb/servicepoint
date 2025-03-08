use crate::{
    commands::check_command_code_only, commands::TryFromPacketError,
    command_code::CommandCode, Packet, TypedCommand,
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
/// # #[allow(deprecated)]
/// connection.send(BitmapLegacyCommand).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[deprecated]
pub struct BitmapLegacyCommand;

#[allow(deprecated)]
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

#[allow(deprecated)]
impl From<BitmapLegacyCommand> for Packet {
    fn from(_: BitmapLegacyCommand) -> Self {
        Packet::command_code_only(CommandCode::BitmapLegacy)
    }
}

#[allow(deprecated)]
impl From<BitmapLegacyCommand> for TypedCommand {
    fn from(command: BitmapLegacyCommand) -> Self {
        Self::BitmapLegacy(command)
    }
}
