use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::errors::TryFromPacketError, BrightnessGrid, ByteGrid, Header,
    Origin, Packet, Tiles, TryIntoPacketError, TypedCommand,
};

/// Set the brightness of individual tiles in a rectangular area of the display.
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct BrightnessGridCommand {
    /// the brightness values per tile
    pub grid: BrightnessGrid,
    /// which tile the brightness rectangle should start
    pub origin: Origin<Tiles>,
}

impl TryFrom<BrightnessGridCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: BrightnessGridCommand) -> Result<Self, Self::Error> {
        Ok(Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::CharBrightness,
        )?)
    }
}

impl TryFrom<&BrightnessGridCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: &BrightnessGridCommand) -> Result<Self, Self::Error> {
        Ok(Packet::origin_grid_as_packet(
            value.origin,
            &value.grid,
            CommandCode::CharBrightness,
        )?)
    }
}

impl From<BrightnessGrid> for BrightnessGridCommand {
    fn from(grid: BrightnessGrid) -> Self {
        Self {
            grid,
            origin: Origin::default(),
        }
    }
}

impl TryFrom<Packet> for BrightnessGridCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code,
                    a: x,
                    b: y,
                    c: width,
                    d: height,
                },
            payload,
        } = packet;

        check_command_code(command_code, CommandCode::CharBrightness)?;

        let expected_size = width as usize * height as usize;
        let payload = match payload {
            None => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    actual: 0,
                    expected: expected_size,
                })
            }
            Some(payload) if payload.len() != expected_size => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    actual: payload.len(),
                    expected: expected_size,
                })
            }
            Some(payload) => payload,
        };

        let grid = ByteGrid::from_raw_parts_unchecked(
            width as usize,
            height as usize,
            payload,
        );
        let grid = match BrightnessGrid::try_from(grid) {
            Ok(grid) => grid,
            Err(val) => return Err(TryFromPacketError::InvalidBrightness(val)),
        };

        Ok(Self {
            grid,
            origin: Origin::new(x as usize, y as usize),
        })
    }
}

impl From<BrightnessGridCommand> for TypedCommand {
    fn from(command: BrightnessGridCommand) -> Self {
        Self::BrightnessGrid(command)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{
            errors::TryFromPacketError,
            tests::{round_trip, TestImplementsCommand},
        },
        Brightness, BrightnessGrid, BrightnessGridCommand, Origin, Packet,
        TypedCommand,
    };

    impl TestImplementsCommand for BrightnessGridCommand {}

    #[test]
    fn round_trip_char_brightness() {
        round_trip(
            BrightnessGridCommand {
                origin: Origin::new(5, 2),
                grid: BrightnessGrid::new(7, 5),
            }
            .into(),
        );
    }

    #[test]
    fn packet_into_char_brightness_invalid() {
        let grid = BrightnessGrid::new(2, 2);
        let command = BrightnessGridCommand {
            origin: Origin::ZERO,
            grid,
        };
        let mut packet: Packet = command.try_into().unwrap();
        let slot = packet.payload.as_mut().unwrap().get_mut(1).unwrap();
        *slot = 23;
        assert_eq!(
            TypedCommand::try_from(packet),
            Err(TryFromPacketError::InvalidBrightness(23))
        );
    }

    #[test]
    fn into_command() {
        let mut grid = BrightnessGrid::new(2, 3);
        grid.iter_mut().enumerate().for_each(|(index, value)| {
            *value = Brightness::saturating_from(index as u8)
        });

        assert_eq!(
            BrightnessGridCommand::from(grid.clone()),
            BrightnessGridCommand {
                grid,
                origin: Origin::default(),
            },
        )
    }

    #[test]
    fn invalid_size() {
        let command: BrightnessGridCommand = BrightnessGrid::new(2, 3).into();
        let packet: Packet = command.try_into().unwrap();
        let packet = Packet {
            header: packet.header,
            payload: Some(packet.payload.as_ref().unwrap()[..5].to_vec()),
        };
        assert_eq!(
            Err(TryFromPacketError::UnexpectedPayloadSize {
                actual: 5,
                expected: 6
            }),
            BrightnessGridCommand::try_from(packet)
        );
    }

    #[test]
    fn missing_payload() {
        let command: BrightnessGridCommand = BrightnessGrid::new(2, 3).into();
        let mut packet: Packet = command.try_into().unwrap();
        packet.payload = None;
        assert_eq!(
            Err(TryFromPacketError::UnexpectedPayloadSize {
                actual: 0,
                expected: 6
            }),
            BrightnessGridCommand::try_from(packet)
        );
    }
}
