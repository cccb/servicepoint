use crate::compression::into_compressed;
use crate::{
    command_code::CommandCode, commands::TryFromPacketError,
    compression::into_decompressed, BitVec, CompressionCode, Header, Offset,
    Packet, TypedCommand,
};

/// Binary operations for use with the [BitVecCommand] command.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum BinaryOperation {
    #[default]
    Overwrite,
    And,
    Or,
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
