//! Raw packet manipulation.
//!
//! Should probably only be used directly to use features not exposed by the library.
//!
//! # Examples
//!
//! Converting a packet to a command and back:
//!
//! ```rust
//! use servicepoint::{Command, packet::Packet};
//! # let command = Command::Clear;
//! let packet: Packet = command.into();
//! let command: Command = Command::try_from(packet).expect("could not read command from packet");
//! ```
//!
//! Converting a packet to bytes and back:
//!
//! ```rust
//! use servicepoint::{Command, packet::Packet};
//! # let command = Command::Clear;
//! # let packet: Packet = command.into();
//! let bytes: Vec<u8> = packet.into();
//! let packet = Packet::try_from(bytes).expect("could not read packet from bytes");
//! ```

use std::mem::size_of;

use crate::compression::into_compressed;
use crate::{
    command_code::CommandCode, Bitmap, Command, CompressionCode, Grid, Offset,
    Origin, Pixels, Tiles, TILE_SIZE,
};

/// A raw header.
///
/// The header specifies the kind of command, the size of the payload and where to display the
/// payload, where applicable.
///
/// Because the meaning of most fields depend on the command, there are no speaking names for them.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Header {
    /// The first two bytes specify which command this packet represents.
    pub command_code: u16,
    /// First command-specific value
    pub a: u16,
    /// Second command-specific value
    pub b: u16,
    /// Third command-specific value
    pub c: u16,
    /// Fourth command-specific value
    pub d: u16,
}

/// The raw payload.
///
/// Should probably only be used directly to use features not exposed by the library.
pub type Payload = Vec<u8>;

/// The raw packet.
///
/// Contents should probably only be used directly to use features not exposed by the library.
///
/// You may want to use [Command] instead.
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    /// Meta-information for the packed command
    pub header: Header,
    /// The data for the packed command
    pub payload: Payload,
}

impl From<Packet> for Vec<u8> {
    /// Turn the packet into raw bytes ready to send
    fn from(value: Packet) -> Self {
        let Packet {
            header:
                Header {
                    command_code: mode,
                    a,
                    b,
                    c,
                    d,
                },
            payload,
        } = value;

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

impl TryFrom<&[u8]> for Packet {
    type Error = ();

    /// Tries to interpret the bytes as a [Packet].
    ///
    /// returns: `Error` if slice is not long enough to be a [Packet]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < size_of::<Header>() {
            return Err(());
        }

        let header = {
            let command_code = Self::u16_from_be_slice(&value[0..=1]);
            let a = Self::u16_from_be_slice(&value[2..=3]);
            let b = Self::u16_from_be_slice(&value[4..=5]);
            let c = Self::u16_from_be_slice(&value[6..=7]);
            let d = Self::u16_from_be_slice(&value[8..=9]);
            Header {
                command_code,
                a,
                b,
                c,
                d,
            }
        };
        let payload = value[10..].to_vec();

        Ok(Packet { header, payload })
    }
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

impl From<Command> for Packet {
    /// Move the [Command] into a [Packet] instance for sending.
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
            Command::CharBrightness(origin, grid) => {
                Self::origin_grid_to_packet(
                    origin,
                    grid,
                    CommandCode::CharBrightness,
                )
            }
            Command::Brightness(brightness) => Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0x00000,
                    b: 0x0000,
                    c: 0x0000,
                    d: 0x0000,
                },
                payload: vec![brightness.into()],
            },
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
            Command::Cp437Data(origin, grid) => Self::origin_grid_to_packet(
                origin,
                grid,
                CommandCode::Cp437Data,
            ),
            Command::Utf8Data(origin, grid) => {
                Self::origin_grid_to_packet(origin, grid, CommandCode::Utf8Data)
            }
        }
    }
}

impl Packet {
    /// Helper method for `BitmapLinear*`-Commands into [Packet]
    #[allow(clippy::cast_possible_truncation)]
    fn bitmap_linear_into_packet(
        command: CommandCode,
        offset: Offset,
        compression: CompressionCode,
        payload: Vec<u8>,
    ) -> Packet {
        let length = payload.len() as u16;
        let payload = into_compressed(compression, payload);
        Packet {
            header: Header {
                command_code: command.into(),
                a: offset as u16,
                b: length,
                c: compression.into(),
                d: 0,
            },
            payload,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn bitmap_win_into_packet(
        origin: Origin<Pixels>,
        pixels: Bitmap,
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

        Packet {
            header: Header {
                command_code: command.into(),
                a: tile_x,
                b: origin.y as u16,
                c: tile_w,
                d: pixel_h,
            },
            payload,
        }
    }

    /// Helper method for creating empty packets only containing the command code
    fn command_code_only(code: CommandCode) -> Packet {
        Packet {
            header: Header {
                command_code: code.into(),
                a: 0x0000,
                b: 0x0000,
                c: 0x0000,
                d: 0x0000,
            },
            payload: vec![],
        }
    }

    fn u16_from_be_slice(slice: &[u8]) -> u16 {
        let mut bytes = [0u8; 2];
        bytes[0] = slice[0];
        bytes[1] = slice[1];
        u16::from_be_bytes(bytes)
    }

    fn origin_grid_to_packet<T>(
        origin: Origin<Tiles>,
        grid: impl Grid<T> + Into<Payload>,
        command_code: CommandCode,
    ) -> Packet {
        Packet {
            header: Header {
                command_code: command_code.into(),
                a: origin.x as u16,
                b: origin.y as u16,
                c: grid.width() as u16,
                d: grid.height() as u16,
            },
            payload: grid.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let p = Packet {
            header: Header {
                command_code: 0,
                a: 1,
                b: 2,
                c: 3,
                d: 4,
            },
            payload: vec![42u8; 23],
        };
        let data: Vec<u8> = p.into();
        let p = Packet::try_from(data).unwrap();
        assert_eq!(
            p,
            Packet {
                header: Header {
                    command_code: 0,
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4
                },
                payload: vec![42u8; 23]
            }
        );
    }

    #[test]
    fn too_small() {
        let data = vec![0u8; 4];
        assert_eq!(Packet::try_from(data.as_slice()), Err(()))
    }
}
