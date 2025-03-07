use crate::{
    commands::{BitmapLinear, TryFromPacketError},
    command_code::CommandCode,
    BitVec, CompressionCode, Offset, Packet, TypedCommand,
};

/// Set pixel data according to an and-mask starting at the offset.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [BitVec] is always uncompressed.
#[derive(Clone, PartialEq, Debug)]
pub struct BitmapLinearAnd {
    /// where to start overwriting pixel data
    pub offset: Offset,
    /// the pixels to send to the display as one long row
    pub bitvec: BitVec,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl TryFrom<Packet> for BitmapLinearAnd {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let (offset, bitvec, compression) =
            BitmapLinear::packet_into_linear_bitmap(packet)?;
        Ok(Self {
            offset,
            bitvec,
            compression,
        })
    }
}

impl From<BitmapLinearAnd> for Packet {
    fn from(bitmap: BitmapLinearAnd) -> Self {
        Packet::bitmap_linear_into_packet(
            CommandCode::BitmapLinearAnd,
            bitmap.offset,
            bitmap.compression,
            bitmap.bitvec.into(),
        )
    }
}

impl From<BitmapLinearAnd> for TypedCommand {
    fn from(command: BitmapLinearAnd) -> Self {
        Self::BitmapLinearAnd(command)
    }
}
