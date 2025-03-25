use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::errors::TryFromPacketError, Brightness, Header, Packet,
    TypedCommand,
};

/// Set the brightness of all tiles to the same value.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// let command = BrightnessCommand { brightness: Brightness::MAX };
/// connection.send(command).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrightnessCommand {
    /// the brightness to set all pixels to
    pub brightness: Brightness,
}

impl From<BrightnessCommand> for Packet {
    fn from(command: BrightnessCommand) -> Self {
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

impl TryFrom<Packet> for BrightnessCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code,
                    a,
                    b,
                    c,
                    d,
                },
            payload,
        } = packet;

        check_command_code(command_code, CommandCode::Brightness)?;

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

impl From<BrightnessCommand> for TypedCommand {
    fn from(command: BrightnessCommand) -> Self {
        Self::Brightness(command)
    }
}

impl From<Brightness> for BrightnessCommand {
    fn from(brightness: Brightness) -> Self {
        BrightnessCommand { brightness }
    }
}

#[cfg(test)]
mod tests {
    use crate::command_code::CommandCode;
    use crate::commands::errors::TryFromPacketError;
    use crate::commands::tests::{round_trip, TestImplementsCommand};
    use crate::{
        commands, Brightness, BrightnessCommand, Header, Packet, TypedCommand,
    };

    impl TestImplementsCommand for BrightnessCommand {}

    #[test]
    fn brightness_as_command() {
        assert_eq!(
            BrightnessCommand {
                brightness: Brightness::MAX
            },
            Brightness::MAX.into()
        );
    }

    #[test]
    fn round_trip_brightness() {
        round_trip(
            BrightnessCommand {
                brightness: Brightness::try_from(6).unwrap(),
            }
            .into(),
        );
    }

    #[test]
    fn error_extraneous_header_values() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Brightness.into(),
                a: 0x00,
                b: 0x13,
                c: 0x37,
                d: 0x00,
            },
            payload: vec![5],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ));
    }

    #[test]
    fn unexpected_payload_size_brightness() {
        assert_eq!(
            TypedCommand::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!()
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 0))
        );

        assert_eq!(
            TypedCommand::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!(0, 0)
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 2))
        );
    }

    #[test]
    fn packet_into_brightness_invalid() {
        let mut packet: Packet = commands::BrightnessCommand {
            brightness: Brightness::MAX,
        }
        .into();
        let slot = packet.payload.get_mut(0).unwrap();
        *slot = 42;
        assert_eq!(
            TypedCommand::try_from(packet),
            Err(TryFromPacketError::InvalidBrightness(42))
        );
    }
}
