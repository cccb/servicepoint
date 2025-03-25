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
/// connection.send(Cp437GridCommand{ origin: Origin::ZERO, grid }).expect("send failed");
/// ```
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// let grid = Cp437Grid::load_ascii("Hello\nWorld", 5, false).unwrap();
/// connection.send(Cp437GridCommand{ origin: Origin::new(2, 2), grid }).unwrap();
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

        let expected_size = width as usize * height as usize;
        if payload.len() != expected_size {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                expected_size,
                payload.len(),
            ));
        }

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
}
