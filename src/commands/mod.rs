//! This module contains the basic commands the display can handle, which all implement [Command].
//!
//! To send a [Command], use a [connection][crate::Connection].
//!
//! # Available commands
//!
//! To send text, take a look at [Cp437GridCommand].
//!
//! To draw pixels, the easiest command to use is [BitmapCommand].
//!
//! The other BitmapLinear-Commands operate on a region of pixel memory directly.
//! [BitVecCommand] overwrites a region.
//! [BitmapLinearOr], [BitmapLinearAnd] and [BitmapLinearXor] apply logical operations per pixel.
//!
//! Out of bounds operations may be truncated or ignored by the display.
//!
//! # Compression
//!
//! Some commands can contain compressed payloads.
//! To get started, use [CompressionCode::default].
//!
//! If you want to archive the best performance (e.g. latency),
//! you can try the different compression algorithms for your hardware and use case.
//!
//! In memory, the payload is not compressed in the [Command].
//! Payload (de-)compression happens when converting the [Command] into a [Packet] or vice versa.
//!
//! # Examples
//!
//! ```rust
//! use servicepoint::*;
//!
//! // create command
//! let command = BrightnessCommand{ brightness: Brightness::MAX };
//!
//! // turn command into Packet
//! let packet: Packet = command.clone().into();
//!
//! // read command from packet
//! let round_tripped = TypedCommand::try_from(packet).unwrap();
//!
//! // round tripping produces exact copy
//! assert_eq!(round_tripped, TypedCommand::from(command.clone()));
//!
//! // send command
//! # let connection = FakeConnection;
//! connection.send(command).unwrap();
//! ```

mod bitmap;
mod bitmap_legacy;
mod bitvec;
mod char_brightness;
mod clear;
mod cp437_data;
mod fade_out;
mod global_brightness;
mod hard_reset;
mod utf8_data;

use crate::command_code::CommandCode;
use crate::*;
use std::fmt::Debug;

pub use bitmap::*;
pub use bitmap_legacy::*;
pub use bitvec::*;
pub use char_brightness::*;
pub use clear::*;
pub use cp437_data::*;
pub use fade_out::*;
pub use global_brightness::*;
pub use hard_reset::*;
pub use utf8_data::*;

/// Represents a command that can be sent to the display.
pub trait Command: Debug + Clone + PartialEq + Into<Packet> {}

impl<T: Debug + Clone + PartialEq + Into<Packet>> Command for T {}

/// This enum contains all commands provided by the library.
/// This is useful in case you want one data type for all kinds of commands without using `dyn`.
///
/// Please look at the contained structs for documentation per command.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum TypedCommand {
    Clear(ClearCommand),

    CharGrid(CharGridCommand),

    Cp437Grid(Cp437GridCommand),

    Bitmap(BitmapCommand),

    Brightness(BrightnessCommand),

    BrightnessGrid(BrightnessGridCommand),

    BitVec(BitVecCommand),

    HardReset(HardResetCommand),

    FadeOut(FadeOutCommand),

    #[allow(deprecated)]
    #[deprecated]
    BitmapLegacy(BitmapLegacyCommand),
}

/// Err values for [Command::try_from].
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TryFromPacketError {
    /// the contained command code does not correspond to a known command
    #[error("The command code {0:?} does not correspond to a known command")]
    InvalidCommand(u16),
    /// the expected payload size was n, but size m was found
    #[error("the expected payload size was {0}, but size {1} was found")]
    UnexpectedPayloadSize(usize, usize),
    /// Header fields not needed for the command have been used.
    ///
    /// Note that these commands would usually still work on the actual display.
    #[error("Header fields not needed for the command have been used")]
    ExtraneousHeaderValues,
    /// The contained compression code is not known. This could be of disabled features.
    #[error("The compression code {0:?} does not correspond to a known compression algorithm.")]
    InvalidCompressionCode(u16),
    /// Decompression of the payload failed. This can be caused by corrupted packets.
    #[error("The decompression of the payload failed")]
    DecompressionFailed,
    /// The given brightness value is out of bounds
    #[error("The given brightness value {0} is out of bounds.")]
    InvalidBrightness(u8),
    /// Some provided text was not valid UTF-8.
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}

impl TryFrom<Packet> for TypedCommand {
    type Error = TryFromPacketError;

    /// Try to interpret the [Packet] as one containing a [Command]
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header: Header { command_code, .. },
            ..
        } = packet;
        let command_code = match CommandCode::try_from(command_code) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCommand(command_code));
            }
            Ok(value) => value,
        };

        Ok(match command_code {
            CommandCode::Clear => {
                TypedCommand::Clear(commands::ClearCommand::try_from(packet)?)
            }
            CommandCode::Brightness => TypedCommand::Brightness(
                commands::BrightnessCommand::try_from(packet)?,
            ),
            CommandCode::HardReset => {
                TypedCommand::HardReset(commands::HardResetCommand::try_from(packet)?)
            }
            CommandCode::FadeOut => {
                TypedCommand::FadeOut(commands::FadeOutCommand::try_from(packet)?)
            }
            CommandCode::Cp437Data => {
                TypedCommand::Cp437Grid(commands::Cp437GridCommand::try_from(packet)?)
            }
            CommandCode::CharBrightness => {
                TypedCommand::BrightnessGrid(commands::BrightnessGridCommand::try_from(packet)?)
            }
            CommandCode::Utf8Data => {
                TypedCommand::CharGrid(commands::CharGridCommand::try_from(packet)?)
            }
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => {
                TypedCommand::BitmapLegacy(commands::BitmapLegacyCommand::try_from(packet)?)
            }
            CommandCode::BitmapLinear
            | CommandCode::BitmapLinearOr
            | CommandCode::BitmapLinearAnd
            | CommandCode::BitmapLinearXor => {
                TypedCommand::BitVec(commands::BitVecCommand::try_from(packet)?)
            }
            CommandCode::BitmapLinearWinUncompressed => {
                TypedCommand::Bitmap(commands::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_zlib")]
            CommandCode::BitmapLinearWinZlib => {
                TypedCommand::Bitmap(commands::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_bzip2")]
            CommandCode::BitmapLinearWinBzip2 => {
                TypedCommand::Bitmap(commands::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_lzma")]
            CommandCode::BitmapLinearWinLzma => {
                TypedCommand::Bitmap(commands::BitmapCommand::try_from(packet)?)
            }
            #[cfg(feature = "compression_zstd")]
            CommandCode::BitmapLinearWinZstd => {
                TypedCommand::Bitmap(commands::BitmapCommand::try_from(packet)?)
            }
        })
    }
}

impl From<TypedCommand> for Packet {
    fn from(command: TypedCommand) -> Self {
        match command {
            TypedCommand::Clear(c) => c.into(),
            TypedCommand::CharGrid(c) => c.into(),
            TypedCommand::Cp437Grid(c) => c.into(),
            TypedCommand::Bitmap(c) => c.into(),
            TypedCommand::Brightness(c) => c.into(),
            TypedCommand::BrightnessGrid(c) => c.into(),
            TypedCommand::BitVec(c) => c.into(),
            TypedCommand::HardReset(c) => c.into(),
            TypedCommand::FadeOut(c) => c.into(),
            #[allow(deprecated)]
            TypedCommand::BitmapLegacy(c) => c.into(),
        }
    }
}

pub(self) fn check_command_code_only(
    packet: Packet,
    code: CommandCode,
) -> Option<TryFromPacketError> {
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
    if packet.header.command_code != u16::from(code) {
        Some(TryFromPacketError::InvalidCommand(
            packet.header.command_code,
        ))
    } else if !payload.is_empty() {
        Some(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
    } else if a != 0 || b != 0 || c != 0 || d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::command_code::CommandCode;
    use crate::commands::{BinaryOperation, TryFromPacketError};
    use crate::*;

    fn round_trip(original: TypedCommand) {
        let packet: Packet = original.clone().into();
        let copy: TypedCommand = match TypedCommand::try_from(packet) {
            Ok(command) => command,
            Err(err) => panic!("could not reload {original:?}: {err:?}"),
        };
        assert_eq!(copy, original);
    }

    fn all_compressions<'t>() -> &'t [CompressionCode] {
        &[
            CompressionCode::Uncompressed,
            #[cfg(feature = "compression_lzma")]
            CompressionCode::Lzma,
            #[cfg(feature = "compression_bzip2")]
            CompressionCode::Bzip2,
            #[cfg(feature = "compression_zlib")]
            CompressionCode::Zlib,
            #[cfg(feature = "compression_zstd")]
            CompressionCode::Zstd,
        ]
    }

    #[test]
    fn round_trip_clear() {
        round_trip(TypedCommand::Clear(commands::ClearCommand));
    }

    #[test]
    fn round_trip_hard_reset() {
        round_trip(TypedCommand::HardReset(commands::HardResetCommand));
    }

    #[test]
    fn round_trip_fade_out() {
        round_trip(TypedCommand::FadeOut(commands::FadeOutCommand));
    }

    #[test]
    fn round_trip_brightness() {
        round_trip(TypedCommand::Brightness(commands::BrightnessCommand {
            brightness: Brightness::try_from(6).unwrap(),
        }));
    }

    #[test]
    #[allow(deprecated)]
    fn round_trip_bitmap_legacy() {
        round_trip(TypedCommand::BitmapLegacy(commands::BitmapLegacyCommand));
    }

    #[test]
    fn round_trip_char_brightness() {
        round_trip(TypedCommand::BrightnessGrid(
            commands::BrightnessGridCommand {
                origin: Origin::new(5, 2),
                grid: BrightnessGrid::new(7, 5),
            },
        ));
    }

    #[test]
    fn round_trip_cp437_data() {
        round_trip(TypedCommand::Cp437Grid(commands::Cp437GridCommand {
            origin: Origin::new(5, 2),
            grid: Cp437Grid::new(7, 5),
        }));
    }

    #[test]
    fn round_trip_utf8_data() {
        round_trip(TypedCommand::CharGrid(commands::CharGridCommand {
            origin: Origin::new(5, 2),
            grid: CharGrid::new(7, 5),
        }));
    }

    #[test]
    fn round_trip_bitmap_linear() {
        for compression in all_compressions().iter().copied() {
            for operation in [
                BinaryOperation::Overwrite,
                BinaryOperation::And,
                BinaryOperation::Or,
                BinaryOperation::Xor,
            ] {
                round_trip(TypedCommand::BitVec(commands::BitVecCommand {
                    offset: 23,
                    bitvec: BitVec::repeat(false, 40),
                    compression,
                    operation,
                }));
            }
            round_trip(TypedCommand::Bitmap(commands::BitmapCommand {
                origin: Origin::ZERO,
                bitmap: Bitmap::max_sized(),
                compression,
            }));
        }
    }

    #[test]
    fn error_invalid_command() {
        let p = Packet {
            header: Header {
                command_code: 0xFF,
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::InvalidCommand(0xFF))
        ))
    }

    #[test]
    fn error_extraneous_header_values_clear() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Clear.into(),
                a: 0x05,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_values_brightness() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Brightness.into(),
                a: 0x00,
                b: 0x13,
                c: 0x37,
                d: 0x00,
            },
            payload: vec![5],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_hard_reset() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::HardReset.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x01,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_fade_out() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x10,
                b: 0x00,
                c: 0x00,
                d: 0x01,
            },
            payload: vec![],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_unexpected_payload() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![5, 7],
        };
        let result = TypedCommand::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::UnexpectedPayloadSize(0, 2))
        ))
    }

    #[test]
    fn error_decompression_failed_win() {
        for compression in all_compressions().iter().copied() {
            let p: Packet = commands::BitmapCommand {
                origin: Origin::new(16, 8),
                bitmap: Bitmap::new(8, 8).unwrap(),
                compression,
            }
            .into();

            let Packet {
                header,
                mut payload,
            } = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet { header, payload };
            let result = TypedCommand::try_from(p);
            if compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn error_decompression_failed_and() {
        for compression in all_compressions().iter().copied() {
            let p: Packet = commands::BitVecCommand {
                offset: 0,
                bitvec: BitVec::repeat(false, 8),
                compression,
                operation: BinaryOperation::Overwrite,
            }
            .into();
            let Packet {
                header,
                mut payload,
            } = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet { header, payload };
            let result = TypedCommand::try_from(p);
            if compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                // when not compressing, there is no way to detect corrupted data
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn unexpected_payload_size_brightness() {
        assert_eq!(
            TypedCommand::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!()
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 0))
        );

        assert_eq!(
            TypedCommand::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!(0, 0)
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 2))
        );
    }

    #[test]
    fn error_reserved_used() {
        let Packet { header, payload } = commands::BitVecCommand {
            offset: 0,
            bitvec: BitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::Or,
        }
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: sub,
            d: _reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: length,
                c: sub,
                d: 69,
            },
            payload,
        };
        assert_eq!(
            TypedCommand::try_from(p),
            Err(TryFromPacketError::ExtraneousHeaderValues)
        );
    }

    #[test]
    fn error_invalid_compression() {
        let Packet { header, payload } = commands::BitVecCommand {
            offset: 0,
            bitvec: BitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::And,
        }
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: _sub,
            d: reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: length,
                c: 42,
                d: reserved,
            },
            payload,
        };
        assert_eq!(
            TypedCommand::try_from(p),
            Err(TryFromPacketError::InvalidCompressionCode(42))
        );
    }

    #[test]
    fn error_unexpected_size() {
        let Packet { header, payload } = commands::BitVecCommand {
            offset: 0,
            bitvec: BitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::Xor,
        }
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: compression,
            d: reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: 420,
                c: compression,
                d: reserved,
            },
            payload,
        };
        assert_eq!(
            TypedCommand::try_from(p),
            Err(TryFromPacketError::UnexpectedPayloadSize(
                420,
                length as usize,
            ))
        );
    }

    #[test]
    fn origin_add() {
        assert_eq!(
            Origin::<Pixels>::new(4, 2),
            Origin::new(1, 0) + Origin::new(3, 2)
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

    #[test]
    fn packet_into_brightness_invalid() {
        let mut packet: Packet = commands::BrightnessCommand {
            brightness: Brightness::MAX,
        }
        .into();
        let slot = packet.payload.get_mut(0).unwrap();
        *slot = 42;
        assert_eq!(
            TypedCommand::try_from(packet),
            Err(TryFromPacketError::InvalidBrightness(42))
        );
    }
}
