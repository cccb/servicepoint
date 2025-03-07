use crate::{
    command::TryFromPacketError, command_code::CommandCode, CharGrid, Header,
    Origin, Packet, Tiles, TypedCommand,
};

/// Show text on the screen.
///
/// The text is sent in the form of a 2D grid of UTF-8 encoded characters (the default encoding in rust).
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = connection::Fake;
/// let grid = CharGrid::from("Hello,\nWorld!");
/// connection.send(command::Utf8Data { origin: Origin::ZERO, grid }).expect("send failed");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Utf8Data {
    /// which tile the text should start
    pub origin: Origin<Tiles>,
    /// the text to send to the display
    pub grid: CharGrid,
}

impl From<Utf8Data> for Packet {
    fn from(value: Utf8Data) -> Self {
        Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::Utf8Data,
        )
    }
}

impl TryFrom<Packet> for Utf8Data {
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
        let payload: Vec<_> =
            String::from_utf8(payload.clone())?.chars().collect();
        Ok(Self {
            origin: Origin::new(a as usize, b as usize),
            grid: CharGrid::load(c as usize, d as usize, &payload),
        })
    }
}

impl From<Utf8Data> for TypedCommand {
    fn from(command: Utf8Data) -> Self {
        Self::Utf8Data(command)
    }
}
