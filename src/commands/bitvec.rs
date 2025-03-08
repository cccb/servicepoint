use crate::compression::into_compressed;
use crate::{
    command_code::CommandCode, commands::TryFromPacketError,
    compression::into_decompressed, BitVec, CompressionCode, Header, Offset,
    Packet, TypedCommand,
};

/// Binary operations for use with the [BitVecCommand] command.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum BinaryOperation {
    /// r := a
    #[default]
    Overwrite,
    /// r := a && b
    And,
    /// r := a || b
    Or,
    /// r := (a || b) && (a != b)
    Xor,
}

/// Set pixel data starting at the pixel offset on screen.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The [BinaryOperation] will be applied on the display comparing old and sent bit.
///
/// `new_bit = old_bit op sent_bit`
///
/// For example, [BinaryOperation::Or] can be used to turn on some pixels without affecting other pixels.
///
/// The contained [BitVec] is always uncompressed.
#[derive(Clone, PartialEq, Debug)]
pub struct BitVecCommand {
    /// where to start overwriting pixel data
    pub offset: Offset,
    /// the pixels to send to the display as one long row
    pub bitvec: BitVec,
    /// The operation to apply on the display per bit comparing old and new state.
    pub operation: BinaryOperation,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl From<BitVecCommand> for Packet {
    fn from(command: BitVecCommand) -> Self {
        let command_code = match command.operation {
            BinaryOperation::Overwrite => CommandCode::BitmapLinear,
            BinaryOperation::And => CommandCode::BitmapLinearAnd,
            BinaryOperation::Or => CommandCode::BitmapLinearOr,
            BinaryOperation::Xor => CommandCode::BitmapLinearXor,
        };

        let payload: Vec<_> = command.bitvec.into();
        let length = payload.len() as u16;
        let payload = into_compressed(command.compression, payload);
        Packet {
            header: Header {
                command_code: command_code.into(),
                a: command.offset as u16,
                b: length,
                c: command.compression.into(),
                d: 0,
            },
            payload,
        }
    }
}

impl TryFrom<Packet> for BitVecCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header:
                Header {
                    command_code,
                    a: offset,
                    b: length,
                    c: sub,
                    d: reserved,
                    ..
                },
            payload,
        } = packet;
        let command_code = CommandCode::try_from(command_code)
            .map_err(|_| TryFromPacketError::InvalidCommand(command_code))?;
        let operation = match command_code {
            CommandCode::BitmapLinear => BinaryOperation::Overwrite,
            CommandCode::BitmapLinearAnd => BinaryOperation::And,
            CommandCode::BitmapLinearOr => BinaryOperation::Or,
            CommandCode::BitmapLinearXor => BinaryOperation::Xor,
            _ => {
                return Err(TryFromPacketError::InvalidCommand(
                    command_code.into(),
                ))
            }
        };

        if reserved != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }
        let compression = match CompressionCode::try_from(sub) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCompressionCode(sub));
            }
            Ok(value) => value,
        };
        let payload = match into_decompressed(compression, payload) {
            None => return Err(TryFromPacketError::DecompressionFailed),
            Some(value) => value,
        };
        if payload.len() != length as usize {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                length as usize,
                payload.len(),
            ));
        }
        Ok(Self {
            offset: offset as Offset,
            bitvec: BitVec::from_vec(payload),
            compression,
            operation,
        })
    }
}

impl From<BitVecCommand> for TypedCommand {
    fn from(command: BitVecCommand) -> Self {
        Self::BitVec(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::tests::round_trip;
    use crate::{commands, Bitmap, BitmapCommand, Origin};

    #[test]
    fn command_code() {
        assert_eq!(
            BitVecCommand::try_from(Packet {
                payload: vec![],
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    ..Default::default()
                }
            }),
            Err(TryFromPacketError::InvalidCommand(
                CommandCode::Brightness.into()
            ))
        );
    }

    #[test]
    fn round_trip_bitmap_linear() {
        for compression in CompressionCode::ALL {
            for operation in [
                BinaryOperation::Overwrite,
                BinaryOperation::And,
                BinaryOperation::Or,
                BinaryOperation::Xor,
            ] {
                round_trip(
                    BitVecCommand {
                        offset: 23,
                        bitvec: BitVec::repeat(false, 40),
                        compression: *compression,
                        operation,
                    }
                    .into(),
                );
            }
            round_trip(
                BitmapCommand {
                    origin: Origin::ZERO,
                    bitmap: Bitmap::max_sized(),
                    compression: *compression,
                }
                .into(),
            );
        }
    }

    #[test]
    fn error_decompression_failed_and() {
        for compression in CompressionCode::ALL {
            let p: Packet = commands::BitVecCommand {
                offset: 0,
                bitvec: BitVec::repeat(false, 8),
                compression: *compression,
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
            if *compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                // when not compressing, there is no way to detect corrupted data
                assert!(result.is_ok());
            }
        }
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
}
