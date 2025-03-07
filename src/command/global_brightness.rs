use crate::{
    command::TryFromPacketError, command_code::CommandCode, Brightness, Header,
    Packet, TypedCommand,
};

/// Set the brightness of all tiles to the same value.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = connection::Fake;
/// let command = command::GlobalBrightness { brightness: Brightness::MAX };
/// connection.send(command).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalBrightness {
    /// the brightness to set all pixels to
    pub brightness: Brightness,
}

impl From<GlobalBrightness> for Packet {
    fn from(command: GlobalBrightness) -> Self {
        Self {
            header: Header {
                command_code: CommandCode::Brightness.into(),
                a: 0x00000,
                b: 0x0000,
                c: 0x0000,
                d: 0x0000,
            },
            payload: vec![command.brightness.into()],
        }
    }
}

impl TryFrom<Packet> for GlobalBrightness {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code: _,
                    a,
                    b,
                    c,
                    d,
                },
            payload,
        } = packet;
        if payload.len() != 1 {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                1,
                payload.len(),
            ));
        }

        if a != 0 || b != 0 || c != 0 || d != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }

        match Brightness::try_from(payload[0]) {
            Ok(brightness) => Ok(Self { brightness }),
            Err(_) => Err(TryFromPacketError::InvalidBrightness(payload[0])),
        }
    }
}

impl From<GlobalBrightness> for TypedCommand {
    fn from(command: GlobalBrightness) -> Self {
        Self::GlobalBrightness(command)
    }
}

impl From<Brightness> for Packet {
    fn from(brightness: Brightness) -> Self {
        Packet::from(GlobalBrightness { brightness })
    }
}
