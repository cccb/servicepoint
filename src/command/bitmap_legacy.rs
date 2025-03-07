use crate::{
    command::check_command_code_only, command::TryFromPacketError,
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
/// # let connection = connection::Fake;
/// // this sends a packet that does nothing
/// # #[allow(deprecated)]
/// connection.send(command::BitmapLegacy).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
#[deprecated]
pub struct BitmapLegacy;

#[allow(deprecated)]
impl TryFrom<Packet> for BitmapLegacy {
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
impl From<BitmapLegacy> for Packet {
    fn from(_: BitmapLegacy) -> Self {
        Packet::command_code_only(CommandCode::BitmapLegacy)
    }
}

#[allow(deprecated)]
impl From<BitmapLegacy> for TypedCommand {
    fn from(command: BitmapLegacy) -> Self {
        Self::BitmapLegacy(command)
    }
}
