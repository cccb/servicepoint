use crate::{
    commands::TryFromPacketError, command_code::CommandCode, BrightnessGrid,
    ByteGrid, Header, Origin, Packet, Tiles, TypedCommand,
};

/// Set the brightness of individual tiles in a rectangular area of the display.
#[derive(Clone, PartialEq, Debug)]
pub struct CharBrightness {
    /// which tile the brightness rectangle should start
    pub origin: Origin<Tiles>,
    /// the brightness values per tile
    pub grid: BrightnessGrid,
}

impl From<CharBrightness> for Packet {
    fn from(value: CharBrightness) -> Self {
        Packet::origin_grid_to_packet(
            value.origin,
            value.grid,
            CommandCode::CharBrightness,
        )
    }
}

impl TryFrom<Packet> for CharBrightness {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code: _,
                    a: x,
                    b: y,
                    c: width,
                    d: height,
                },
            payload,
        } = packet;

        let grid = ByteGrid::load(width as usize, height as usize, &*payload);
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

impl From<CharBrightness> for TypedCommand {
    fn from(command: CharBrightness) -> Self {
        Self::CharBrightness(command)
    }
}
