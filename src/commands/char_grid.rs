use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::errors::TryFromPacketError, CharGrid, Header, Origin, Packet,
    Tiles, TryIntoPacketError, TypedCommand,
};

/// Show text on the screen.
///
/// The text is sent in the form of a 2D grid of UTF-8 encoded characters (the default encoding in rust).
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// let grid = CharGrid::from("Hello,\nWorld!");
/// connection.send(CharGridCommand { origin: Origin::ZERO, grid }).expect("send failed");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharGridCommand {
    /// the text to send to the display
    pub grid: CharGrid,
    /// which tile the text should start on
    pub origin: Origin<Tiles>,
}

impl TryFrom<CharGridCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: CharGridCommand) -> Result<Self, Self::Error> {
        Ok(Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::Utf8Data,
        )?)
    }
}

impl TryFrom<Packet> for CharGridCommand {
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

        check_command_code(command_code, CommandCode::Utf8Data)?;

        let payload: Vec<_> =
            String::from_utf8(payload.clone())?.chars().collect();

        let expected = width as usize * height as usize;
        if payload.len() != expected {
            return Err(TryFromPacketError::UnexpectedPayloadSize {
                expected,
                actual: payload.len(),
            });
        }

        Ok(Self {
            origin: Origin::new(origin_x as usize, origin_y as usize),
            grid: CharGrid::from_raw_parts_unchecked(
                width as usize,
                height as usize,
                payload,
            ),
        })
    }
}

impl From<CharGridCommand> for TypedCommand {
    fn from(command: CharGridCommand) -> Self {
        Self::CharGrid(command)
    }
}

impl From<CharGrid> for CharGridCommand {
    fn from(grid: CharGrid) -> Self {
        Self {
            grid,
            origin: Origin::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::tests::{round_trip, TestImplementsCommand},
        CharGrid, CharGridCommand, Origin, Packet, TryFromPacketError,
    };

    impl TestImplementsCommand for CharGridCommand {}

    #[test]
    fn round_trip_utf8_data() {
        round_trip(
            CharGridCommand {
                origin: Origin::new(5, 2),
                grid: CharGrid::new(7, 5),
            }
            .into(),
        );
    }

    #[test]
    #[cfg(feature = "cp437")]
    fn into_command() {
        let mut grid = CharGrid::new(2, 3);
        grid.iter_mut().enumerate().for_each(|(index, value)| {
            *value = crate::cp437::cp437_to_char(index as u8)
        });

        assert_eq!(
            CharGridCommand::from(grid.clone()),
            CharGridCommand {
                grid,
                origin: Origin::default(),
            },
        )
    }

    #[test]
    fn invalid_size() {
        let command: CharGridCommand = CharGrid::new(2, 3).into();
        let packet: Packet = command.try_into().unwrap();
        let packet = Packet {
            header: packet.header,
            payload: packet.payload[..5].to_vec(),
        };
        assert_eq!(
            Err(TryFromPacketError::UnexpectedPayloadSize {
                actual: 5,
                expected: 6
            }),
            CharGridCommand::try_from(packet)
        );
    }
}
