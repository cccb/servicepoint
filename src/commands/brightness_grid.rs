use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::TryFromPacketError, BrightnessGrid, ByteGrid, Header, Origin,
    Packet, Tiles, TypedCommand,
};

/// Set the brightness of individual tiles in a rectangular area of the display.
#[derive(Clone, PartialEq, Debug)]
pub struct BrightnessGridCommand {
    /// which tile the brightness rectangle should start
    pub origin: Origin<Tiles>,
    /// the brightness values per tile
    pub grid: BrightnessGrid,
}

impl From<BrightnessGridCommand> for Packet {
    fn from(value: BrightnessGridCommand) -> Self {
        Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::CharBrightness,
        )
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
        if payload.len() != expected_size {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                payload.len(),
                expected_size,
            ));
        }

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
    use crate::commands::tests::round_trip;
    use crate::{
        commands, BrightnessGrid, BrightnessGridCommand, Origin, Packet,
        TryFromPacketError, TypedCommand,
    };

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
        let command = commands::BrightnessGridCommand {
            origin: Origin::ZERO,
            grid,
        };
        let mut packet: Packet = command.into();
        let slot = packet.payload.get_mut(1).unwrap();
        *slot = 23;
        assert_eq!(
            TypedCommand::try_from(packet),
            Err(TryFromPacketError::InvalidBrightness(23))
        );
    }
}
