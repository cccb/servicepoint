use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::errors::TryFromPacketError, Cp437Grid, Header, Origin, Packet,
    Tiles, TryIntoPacketError, TypedCommand,
};

/// Show text on the screen.
///
/// The text is sent in the form of a 2D grid of [CP-437] encoded characters.
///
/// <div class="warning">You probably want to use [Command::Utf8Data] instead</div>
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// let grid = CharGrid::from("Hello,\nWorld!");
/// let grid = Cp437Grid::from(&grid);
/// connection.send_command(Cp437GridCommand{ origin: Origin::ZERO, grid }).expect("send failed");
/// ```
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// let grid = Cp437Grid::load_ascii("Hello\nWorld", 5, false).unwrap();
/// connection.send_command(Cp437GridCommand{ origin: Origin::new(2, 2), grid }).unwrap();
/// ```
/// [CP-437]: https://en.wikipedia.org/wiki/Code_page_437
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cp437GridCommand {
    /// the text to send to the display
    pub grid: Cp437Grid,
    /// which tile the text should start
    pub origin: Origin<Tiles>,
}

impl TryFrom<Cp437GridCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: Cp437GridCommand) -> Result<Self, Self::Error> {
        Ok(Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::Cp437Data,
        )?)
    }
}

impl TryFrom<Packet> for Cp437GridCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code,
                    a: origin_x,
                    b: origin_y,
                    c: width,
                    d: height,
                },
            payload,
        } = packet;

        check_command_code(command_code, CommandCode::Cp437Data)?;

        let expected = width as usize * height as usize;
        let payload = match payload {
            None => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    expected,
                    actual: 0,
                })
            }
            Some(payload) if payload.len() != expected => {
                return Err(TryFromPacketError::UnexpectedPayloadSize {
                    expected,
                    actual: payload.len(),
                })
            }
            Some(payload) => payload,
        };

        Ok(Self {
            origin: Origin::new(origin_x as usize, origin_y as usize),
            grid: Cp437Grid::from_raw_parts_unchecked(
                width as usize,
                height as usize,
                payload,
            ),
        })
    }
}

impl From<Cp437GridCommand> for TypedCommand {
    fn from(command: Cp437GridCommand) -> Self {
        Self::Cp437Grid(command)
    }
}

impl From<Cp437Grid> for Cp437GridCommand {
    fn from(grid: Cp437Grid) -> Self {
        Self {
            grid,
            origin: Origin::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::tests::{round_trip, TestImplementsCommand};

    impl TestImplementsCommand for Cp437GridCommand {}

    #[test]
    fn round_trip_cp437_data() {
        round_trip(
            Cp437GridCommand {
                origin: Origin::new(5, 2),
                grid: Cp437Grid::new(7, 5),
            }
            .into(),
        );
    }

    #[test]
    fn into_command() {
        let mut grid = Cp437Grid::new(2, 3);
        grid.iter_mut()
            .enumerate()
            .for_each(|(index, value)| *value = index as u8);

        assert_eq!(
            Cp437GridCommand::from(grid.clone()),
            Cp437GridCommand {
                grid,
                origin: Origin::default(),
            },
        )
    }

    #[test]
    fn invalid_size() {
        let command: Cp437GridCommand = Cp437Grid::new(2, 3).into();
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
            Cp437GridCommand::try_from(packet)
        );
    }

    #[test]
    fn missing_payload() {
        let command: Cp437GridCommand = Cp437Grid::new(2, 3).into();
        let mut packet: Packet = command.try_into().unwrap();
        packet.payload = None;
        assert_eq!(
            Err(TryFromPacketError::UnexpectedPayloadSize {
                actual: 0,
                expected: 6
            }),
            Cp437GridCommand::try_from(packet)
        );
    }
}
