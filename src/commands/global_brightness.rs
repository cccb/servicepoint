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
/// let command = GlobalBrightnessCommand { brightness: Brightness::MAX };
/// connection.send_command(command).unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalBrightnessCommand {
    /// the brightness to set all pixels to
    pub brightness: Brightness,
}

impl From<GlobalBrightnessCommand> for Packet {
    fn from(value: GlobalBrightnessCommand) -> Self {
        Packet::from(&value)
    }
}

impl From<&GlobalBrightnessCommand> for Packet {
    fn from(command: &GlobalBrightnessCommand) -> Self {
        Self {
            header: Header {
                command_code: CommandCode::Brightness.into(),
                a: 0x00000,
                b: 0x0000,
                c: 0x0000,
                d: 0x0000,
            },
            payload: Some(vec![command.brightness.into()]),
        }
    }
}

impl TryFrom<Packet> for GlobalBrightnessCommand {
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

        if a != 0 || b != 0 || c != 0 || d != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }

        let brightness = match payload {
            None => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    expected: 1,
                    actual: 0,
                })
            }
            Some(payload) if payload.len() == 1 => payload[0],
            Some(payload) => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    expected: 1,
                    actual: payload.len(),
                });
            }
        };

        match Brightness::try_from(brightness) {
            Ok(brightness) => Ok(Self { brightness }),
            Err(_) => Err(TryFromPacketError::InvalidBrightness(brightness)),
        }
    }
}

impl From<GlobalBrightnessCommand> for TypedCommand {
    fn from(command: GlobalBrightnessCommand) -> Self {
        Self::Brightness(command)
    }
}

impl From<Brightness> for GlobalBrightnessCommand {
    fn from(brightness: Brightness) -> Self {
        GlobalBrightnessCommand { brightness }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        command_code::CommandCode,
        commands::{errors::TryFromPacketError, tests::TestImplementsCommand},
        Brightness, GlobalBrightnessCommand, Header, Packet, TypedCommand,
    };

    impl TestImplementsCommand for GlobalBrightnessCommand {}

    #[test]
    fn brightness_as_command() {
        assert_eq!(
            GlobalBrightnessCommand {
                brightness: Brightness::MAX
            },
            Brightness::MAX.into()
        );
    }

    #[test]
    fn round_trip() {
        crate::commands::tests::round_trip(
            GlobalBrightnessCommand {
                brightness: Brightness::try_from(6).unwrap(),
            }
            .into(),
        );
    }

    #[test]
    fn round_trip_ref() {
        crate::commands::tests::round_trip_ref(
            &GlobalBrightnessCommand {
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
            payload: Some(vec![5]),
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
                payload: None
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize {
                expected: 1,
                actual: 0
            })
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
                payload: Some(vec!(0, 0))
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize {
                expected: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn packet_into_brightness_invalid() {
        let mut packet: Packet = GlobalBrightnessCommand {
            brightness: Brightness::MAX,
        }
        .into();
        let slot = packet.payload.as_mut().unwrap().get_mut(0).unwrap();
        *slot = 42;
        assert_eq!(
            TypedCommand::try_from(packet),
            Err(TryFromPacketError::InvalidBrightness(42))
        );
    }

    #[test]
    fn into_command() {
        assert_eq!(
            GlobalBrightnessCommand::from(Brightness::MIN),
            GlobalBrightnessCommand {
                brightness: Brightness::MIN,
            },
        )
    }
}
