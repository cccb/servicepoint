use crate::{
    command_code::CommandCode, commands::check_command_code,
    commands::TryFromPacketError, CharGrid, Header, Origin, Packet, Tiles,
    TypedCommand,
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
#[derive(Debug, Clone, PartialEq)]
pub struct CharGridCommand {
    /// which tile the text should start
    pub origin: Origin<Tiles>,
    /// the text to send to the display
    pub grid: CharGrid,
}

impl From<CharGridCommand> for Packet {
    fn from(value: CharGridCommand) -> Self {
        Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::Utf8Data,
        )
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

        let expected_size = width as usize * height as usize;
        if payload.len() != expected_size {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                expected_size,
                payload.len(),
            ));
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

#[cfg(test)]
mod tests {
    use crate::commands::tests::round_trip;
    use crate::{CharGrid, CharGridCommand, Origin};

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
}
