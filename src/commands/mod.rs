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
mod typed;

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
pub use typed::*;

/// Represents a command that can be sent to the display.
pub trait Command: Debug + Clone + PartialEq + Into<Packet> {}

impl<T: Debug + Clone + PartialEq + Into<Packet>> Command for T {}

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
