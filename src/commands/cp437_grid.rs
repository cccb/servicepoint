use crate::{
    commands::TryFromPacketError, command_code::CommandCode, Cp437Grid, Header,
    Origin, Packet, Tiles, TypedCommand,
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
#[derive(Clone, Debug, PartialEq)]
pub struct Cp437GridCommand {
    /// which tile the text should start
    pub origin: Origin<Tiles>,
    /// the text to send to the display
    pub grid: Cp437Grid,
}

impl From<Cp437GridCommand> for Packet {
    fn from(value: Cp437GridCommand) -> Self {
        Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::Cp437Data,
        )
    }
}

impl TryFrom<Packet> for Cp437GridCommand {
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
        Ok(Self {
            origin: Origin::new(a as usize, b as usize),
            grid: Cp437Grid::load(c as usize, d as usize, &payload),
        })
    }
}

impl From<Cp437GridCommand> for TypedCommand {
    fn from(command: Cp437GridCommand) -> Self {
        Self::Cp437Grid(command)
    }
}
