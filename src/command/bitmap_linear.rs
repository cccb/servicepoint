use crate::{
    command::TryFromPacketError, command_code::CommandCode,
    compression::into_decompressed, BitVec, CompressionCode, Header, Offset,
    Packet, TypedCommand,
};

/// Set pixel data starting at the pixel offset on screen.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [BitVec] is always uncompressed.
#[derive(Clone, PartialEq, Debug)]
pub struct BitmapLinear {
    /// where to start overwriting pixel data 
    pub offset: Offset,
    /// the pixels to send to the display as one long row
    pub bitvec: BitVec,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl From<BitmapLinear> for Packet {
    fn from(bitmap: BitmapLinear) -> Self {
        Packet::bitmap_linear_into_packet(
            CommandCode::BitmapLinear,
            bitmap.offset,
            bitmap.compression,
            bitmap.bitvec.into(),
        )
    }
}

impl TryFrom<Packet> for BitmapLinear {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let (offset, bitvec, compression) =
            Self::packet_into_linear_bitmap(packet)?;
        Ok(Self {
            offset,
            bitvec,
            compression,
        })
    }
}

impl From<BitmapLinear> for TypedCommand {
    fn from(command: BitmapLinear) -> Self {
        Self::BitmapLinear(command)
    }
}

impl BitmapLinear {
    /// Helper method for Packets into `BitmapLinear*`-Commands
    pub(crate) fn packet_into_linear_bitmap(
        packet: Packet,
    ) -> Result<(Offset, BitVec, CompressionCode), TryFromPacketError> {
        let Packet {
            header:
                Header {
                    a: offset,
                    b: length,
                    c: sub,
                    d: reserved,
                    ..
                },
            payload,
        } = packet;
        if reserved != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }
        let sub = match CompressionCode::try_from(sub) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCompressionCode(sub));
            }
            Ok(value) => value,
        };
        let payload = match into_decompressed(sub, payload) {
            None => return Err(TryFromPacketError::DecompressionFailed),
            Some(value) => value,
        };
        if payload.len() != length as usize {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                length as usize,
                payload.len(),
            ));
        }
        Ok((offset as Offset, BitVec::from_vec(payload), sub))
    }
}
