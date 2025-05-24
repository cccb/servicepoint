use crate::{
    command_code::{CommandCode, InvalidCommandCodeError},
    commands::errors::TryFromPacketError,
    compression::{compress, decompress, CompressionError},
    CompressionCode, DisplayBitVec, Header, Offset, Packet, TryIntoPacketError,
    TypedCommand,
};

/// Binary operations for use with the [`BitVecCommand`] command.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
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
/// The [`BinaryOperation`] will be applied on the display comparing old and sent bit.
///
/// `new_bit = old_bit op sent_bit`
///
/// For example, [`BinaryOperation::Or`] can be used to turn on some pixels without affecting other pixels.
///
/// The contained [`DisplayBitVec`] is always uncompressed.
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct BitVecCommand {
    /// the pixels to send to the display as one long row
    pub bitvec: DisplayBitVec,
    /// where to start overwriting pixel data
    pub offset: Offset,
    /// The operation to apply on the display per bit comparing old and new state.
    pub operation: BinaryOperation,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl TryFrom<BitVecCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: BitVecCommand) -> Result<Self, Self::Error> {
        Packet::try_from(&value)
    }
}

impl TryFrom<&BitVecCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: &BitVecCommand) -> Result<Self, Self::Error> {
        let command_code = match value.operation {
            BinaryOperation::Overwrite => CommandCode::BitmapLinear,
            BinaryOperation::And => CommandCode::BitmapLinearAnd,
            BinaryOperation::Or => CommandCode::BitmapLinearOr,
            BinaryOperation::Xor => CommandCode::BitmapLinearXor,
        };

        let data_ref = value.bitvec.as_raw_slice();
        let length = data_ref.len().try_into()?;
        let payload = match compress(value.compression, data_ref) {
            Ok(payload) => payload,
            Err(CompressionError::NoCompression) => data_ref.to_vec(),
            Err(_) => return Err(TryIntoPacketError::CompressionFailed),
        };
        Ok(Packet {
            header: Header {
                command_code: command_code.into(),
                a: value.offset.try_into()?,
                b: length,
                c: value.compression.into(),
                d: 0,
            },
            payload: Some(payload),
        })
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
                    b: expected_len,
                    c: sub,
                    d: reserved,
                    ..
                },
            payload,
        } = packet;
        let command_code = CommandCode::try_from(command_code)?;
        let operation = match command_code {
            CommandCode::BitmapLinear => BinaryOperation::Overwrite,
            CommandCode::BitmapLinearAnd => BinaryOperation::And,
            CommandCode::BitmapLinearOr => BinaryOperation::Or,
            CommandCode::BitmapLinearXor => BinaryOperation::Xor,
            _ => {
                return Err(InvalidCommandCodeError(command_code.into()).into());
            }
        };

        if reserved != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }
        let compression = CompressionCode::try_from(sub)?;
        let payload =
            payload.ok_or(TryFromPacketError::UnexpectedPayloadSize {
                expected: expected_len as usize,
                actual: 0,
            })?;
        let payload = match decompress(compression, &payload) {
            Ok(payload) => payload,
            Err(CompressionError::NoCompression) => payload.clone(),
            Err(_) => return Err(TryFromPacketError::DecompressionFailed),
        };
        if payload.len() != expected_len as usize {
            return Err(TryFromPacketError::UnexpectedPayloadSize {
                expected: expected_len as usize,
                actual: payload.len(),
            });
        }
        Ok(Self {
            offset: offset as Offset,
            bitvec: DisplayBitVec::from_vec(payload),
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

impl From<DisplayBitVec> for BitVecCommand {
    fn from(bitvec: DisplayBitVec) -> Self {
        Self {
            bitvec,
            operation: BinaryOperation::default(),
            offset: Offset::default(),
            compression: CompressionCode::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        commands, commands::tests::TestImplementsCommand,
        compression_code::InvalidCompressionCodeError, PIXEL_WIDTH,
    };

    impl TestImplementsCommand for BitVecCommand {}

    #[test]
    fn command_code() {
        assert_eq!(
            BitVecCommand::try_from(Packet {
                payload: None,
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    ..Default::default()
                }
            }),
            Err(InvalidCommandCodeError(CommandCode::Brightness.into()).into())
        );
    }

    #[test]
    fn round_trip() {
        for compression in CompressionCode::ALL {
            for operation in [
                BinaryOperation::Overwrite,
                BinaryOperation::And,
                BinaryOperation::Or,
                BinaryOperation::Xor,
            ] {
                crate::commands::tests::round_trip(
                    BitVecCommand {
                        offset: 23,
                        bitvec: DisplayBitVec::repeat(false, 40),
                        compression: *compression,
                        operation,
                    }
                    .into(),
                );
            }
        }
    }

    #[test]
    fn round_trip_ref() {
        for compression in CompressionCode::ALL {
            for operation in [
                BinaryOperation::Overwrite,
                BinaryOperation::And,
                BinaryOperation::Or,
                BinaryOperation::Xor,
            ] {
                crate::commands::tests::round_trip(
                    BitVecCommand {
                        offset: 23,
                        bitvec: DisplayBitVec::repeat(false, 40),
                        compression: *compression,
                        operation,
                    }
                    .into(),
                );
            }
        }
    }

    #[test]
    fn error_decompression_failed_and() {
        for compression in CompressionCode::ALL {
            let p: Packet = commands::BitVecCommand {
                offset: 0,
                bitvec: DisplayBitVec::repeat(false, 8),
                compression: *compression,
                operation: BinaryOperation::Overwrite,
            }
            .try_into()
            .unwrap();
            let Packet { header, payload } = p;

            let mut payload = payload.unwrap();
            // mangle it
            for byte in &mut payload {
                *byte -= *byte / 2;
            }

            let p = Packet {
                header,
                payload: Some(payload),
            };
            let result = TypedCommand::try_from(p);
            if *compression != CompressionCode::Uncompressed {
                assert_eq!(
                    result,
                    Err(TryFromPacketError::DecompressionFailed)
                );
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
            bitvec: DisplayBitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::Or,
        }
        .try_into()
        .unwrap();
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
            bitvec: DisplayBitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::And,
        }
        .try_into()
        .unwrap();
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
            Err(InvalidCompressionCodeError(42).into())
        );
    }

    #[test]
    fn error_unexpected_size() {
        let Packet { header, payload } = commands::BitVecCommand {
            offset: 0,
            bitvec: DisplayBitVec::repeat(false, 8),
            compression: CompressionCode::Uncompressed,
            operation: BinaryOperation::Xor,
        }
        .try_into()
        .unwrap();
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
            Err(TryFromPacketError::UnexpectedPayloadSize {
                expected: 420,
                actual: length as usize,
            })
        );
    }

    #[test]
    fn into_command() {
        let mut bitvec = DisplayBitVec::repeat(true, PIXEL_WIDTH);
        bitvec.fill(true);

        assert_eq!(
            BitVecCommand::from(bitvec.clone()),
            BitVecCommand {
                bitvec,
                offset: 0,
                compression: CompressionCode::default(),
                operation: BinaryOperation::Overwrite,
            },
        )
    }

    #[test]
    fn into_packet_invalid_alignment() {
        let mut cmd = BitVecCommand::from(DisplayBitVec::repeat(false, 32));
        cmd.offset = 5;
        cmd.compression = CompressionCode::Uncompressed;
        let packet = Packet::try_from(cmd).unwrap();
        assert_eq!(
            packet.header,
            Header {
                command_code: 18,
                a: 5,
                b: 4,
                c: 0,
                d: 0
            }
        );

        let cmd = BitVecCommand {
            bitvec: DisplayBitVec::repeat(false, 32),
            offset: 11,
            operation: BinaryOperation::Overwrite,
            compression: CompressionCode::Uncompressed,
        };
        let packet = Packet::try_from(cmd).unwrap();
        assert_eq!(
            packet.header,
            Header {
                command_code: 18,
                a: 11,
                b: 4,
                c: 0,
                d: 0
            }
        );
    }
}
