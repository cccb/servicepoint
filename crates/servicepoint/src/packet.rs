use std::mem::size_of;

use crate::command_code::CommandCode;
use crate::compression::into_compressed;
use crate::{
    Command, CompressionCode, Grid, Offset, Origin, PixelGrid, Pixels,
    TILE_SIZE,
};

/// A raw header. Should probably not be used directly.
#[derive(Debug, PartialEq)]
pub struct Header(pub u16, pub u16, pub u16, pub u16, pub u16);

/// The raw payload. Should probably not be used directly.
pub type Payload = Vec<u8>;

/// The raw packet. Should probably not be used directly.
#[derive(Debug, PartialEq)]
pub struct Packet(pub Header, pub Payload);

impl From<Packet> for Vec<u8> {
    /// Turn the packet into raw bytes ready to send
    fn from(value: Packet) -> Self {
        let Packet(Header(mode, a, b, c, d), payload) = value;

        let mut packet = vec![0u8; 10 + payload.len()];
        packet[0..=1].copy_from_slice(&u16::to_be_bytes(mode));
        packet[2..=3].copy_from_slice(&u16::to_be_bytes(a));
        packet[4..=5].copy_from_slice(&u16::to_be_bytes(b));
        packet[6..=7].copy_from_slice(&u16::to_be_bytes(c));
        packet[8..=9].copy_from_slice(&u16::to_be_bytes(d));

        packet[10..].copy_from_slice(&payload);

        packet
    }
}

fn u16_from_be_slice(slice: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    bytes[0] = slice[0];
    bytes[1] = slice[1];
    u16::from_be_bytes(bytes)
}

impl TryFrom<&[u8]> for Packet {
    type Error = ();

    /// Tries to interpret the bytes as a `Packet`.
    ///
    /// returns: `Error` if slice is not long enough to be a `Packet`
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < size_of::<Header>() {
            return Err(());
        }

        let mode = u16_from_be_slice(&value[0..=1]);
        let a = u16_from_be_slice(&value[2..=3]);
        let b = u16_from_be_slice(&value[4..=5]);
        let c = u16_from_be_slice(&value[6..=7]);
        let d = u16_from_be_slice(&value[8..=9]);
        let payload = value[10..].to_vec();

        Ok(Packet(Header(mode, a, b, c, d), payload))
    }
}

impl From<Command> for Packet {
    /// Move the `Command` into a `Packet` instance for sending.
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: Command) -> Self {
        match value {
            Command::Clear => Self::command_code_only(CommandCode::Clear),
            Command::FadeOut => Self::command_code_only(CommandCode::FadeOut),
            Command::HardReset => {
                Self::command_code_only(CommandCode::HardReset)
            }
            #[allow(deprecated)]
            Command::BitmapLegacy => {
                Self::command_code_only(CommandCode::BitmapLegacy)
            }
            Command::CharBrightness(origin, grid) => Packet(
                Header(
                    CommandCode::CharBrightness.into(),
                    origin.x as u16,
                    origin.y as u16,
                    grid.width() as u16,
                    grid.height() as u16,
                ),
                grid.into(),
            ),
            Command::Brightness(brightness) => Packet(
                Header(
                    CommandCode::Brightness.into(),
                    0x00000,
                    0x0000,
                    0x0000,
                    0x0000,
                ),
                vec![brightness.into()],
            ),
            Command::BitmapLinearWin(origin, pixels, compression) => {
                Self::bitmap_win_into_packet(origin, pixels, compression)
            }
            Command::BitmapLinear(offset, bits, compression) => {
                Self::bitmap_linear_into_packet(
                    CommandCode::BitmapLinear,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearAnd(offset, bits, compression) => {
                Self::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearAnd,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearOr(offset, bits, compression) => {
                Self::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearOr,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearXor(offset, bits, compression) => {
                Self::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearXor,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::Cp437Data(origin, grid) => Packet(
                Header(
                    CommandCode::Cp437Data.into(),
                    origin.x as u16,
                    origin.y as u16,
                    grid.width() as u16,
                    grid.height() as u16,
                ),
                grid.into(),
            ),
        }
    }
}

impl Packet {
    /// Helper method for `BitMapLinear*`-Commands into `Packet`
    #[allow(clippy::cast_possible_truncation)]
    fn bitmap_linear_into_packet(
        command: CommandCode,
        offset: Offset,
        compression: CompressionCode,
        payload: Vec<u8>,
    ) -> Packet {
        let length = payload.len() as u16;
        let payload = into_compressed(compression, payload);
        Packet(
            Header(
                command.into(),
                offset as u16,
                length,
                compression.into(),
                0,
            ),
            payload,
        )
    }

    #[allow(clippy::cast_possible_truncation)]
    fn bitmap_win_into_packet(
        origin: Origin<Pixels>,
        pixels: PixelGrid,
        compression: CompressionCode,
    ) -> Packet {
        debug_assert_eq!(origin.x % 8, 0);
        debug_assert_eq!(pixels.width() % 8, 0);

        let tile_x = (origin.x / TILE_SIZE) as u16;
        let tile_w = (pixels.width() / TILE_SIZE) as u16;
        let pixel_h = pixels.height() as u16;
        let payload = into_compressed(compression, pixels.into());
        let command = match compression {
            CompressionCode::Uncompressed => {
                CommandCode::BitmapLinearWinUncompressed
            }
            #[cfg(feature = "compression_zlib")]
            CompressionCode::Zlib => CommandCode::BitmapLinearWinZlib,
            #[cfg(feature = "compression_bzip2")]
            CompressionCode::Bzip2 => CommandCode::BitmapLinearWinBzip2,
            #[cfg(feature = "compression_lzma")]
            CompressionCode::Lzma => CommandCode::BitmapLinearWinLzma,
            #[cfg(feature = "compression_zstd")]
            CompressionCode::Zstd => CommandCode::BitmapLinearWinZstd,
        };

        Packet(
            Header(command.into(), tile_x, origin.y as u16, tile_w, pixel_h),
            payload,
        )
    }

    /// Helper method for creating empty packets only containing the command code
    fn command_code_only(code: CommandCode) -> Packet {
        Packet(Header(code.into(), 0x0000, 0x0000, 0x0000, 0x0000), vec![])
    }
}

#[cfg(test)]
mod tests {
    use crate::{Header, Packet};

    #[test]
    fn round_trip() {
        let p = Packet(Header(0, 1, 2, 3, 4), vec![42u8; 23]);
        let data: Vec<u8> = p.into();
        let p = Packet::try_from(&*data).unwrap();
        assert_eq!(p, Packet(Header(0, 1, 2, 3, 4), vec![42u8; 23]));
    }

    #[test]
    fn too_small() {
        let data = vec![0u8; 4];
        assert_eq!(Packet::try_from(data.as_slice()), Err(()))
    }
}
