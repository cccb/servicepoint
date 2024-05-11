use crate::command_codes::{CommandCode, CompressionCode};
use crate::compression::{into_compressed, into_decompressed};
use crate::{BitVec, ByteGrid, Header, Packet, PixelGrid, TILE_SIZE};

/// An origin marks the top left position of the
/// data sent to the display.
#[derive(Debug, Clone, Copy)]
pub struct Origin(pub u16, pub u16);

impl Origin {
    pub fn top_left() -> Self {
        Self(0, 0)
    }
}

/// Size defines the width and height of a window
#[derive(Debug, Clone, Copy)]
pub struct Size(pub u16, pub u16);

type Offset = u16;

type Brightness = u8;

#[derive(Debug)]
pub enum Command {
    Clear,
    HardReset,
    FadeOut,
    CharBrightness(Origin, ByteGrid),
    Brightness(Brightness),
    #[deprecated]
    BitmapLegacy,
    BitmapLinear(Offset, BitVec, CompressionCode),
    BitmapLinearAnd(Offset, BitVec, CompressionCode),
    BitmapLinearOr(Offset, BitVec, CompressionCode),
    BitmapLinearXor(Offset, BitVec, CompressionCode),
    Cp437Data(Origin, ByteGrid),
    BitmapLinearWin(Origin, PixelGrid),
}

impl Into<Packet> for Command {
    fn into(self) -> Packet {
        match self {
            Command::Clear => command_code_only(CommandCode::Clear),
            Command::FadeOut => command_code_only(CommandCode::FadeOut),
            Command::HardReset => command_code_only(CommandCode::HardReset),
            #[allow(deprecated)]
            Command::BitmapLegacy => {
                command_code_only(CommandCode::BitmapLegacy)
            }
            Command::CharBrightness(origin, grid) => origin_size_payload(
                CommandCode::CharBrightness,
                origin,
                Size(grid.width as u16, grid.height as u16),
                grid.into(),
            ),
            Command::Brightness(brightness) => Packet(
                Header(
                    CommandCode::Brightness.to_primitive(),
                    0x00000,
                    0x0000,
                    0x0000,
                    0x0000,
                ),
                vec![brightness],
            ),
            Command::BitmapLinearWin(Origin(pixel_x, pixel_y), pixels) => {
                debug_assert_eq!(pixel_x % 8, 0);
                debug_assert_eq!(pixels.width % 8, 0);
                Packet(
                    Header(
                        CommandCode::BitmapLinearWin.to_primitive(),
                        pixel_x / TILE_SIZE,
                        pixel_y,
                        pixels.width as u16 / TILE_SIZE,
                        pixels.height as u16,
                    ),
                    pixels.into(),
                )
            }
            Command::BitmapLinear(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinear,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearAnd(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearAnd,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearOr(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearOr,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearXor(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearXor,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::Cp437Data(origin, grid) => origin_size_payload(
                CommandCode::Cp437Data,
                origin,
                Size(grid.width as u16, grid.height as u16),
                grid.into(),
            ),
        }
    }
}

#[derive(Debug)]
pub enum TryFromPacketError {
    InvalidCommand(u16),
    UnexpectedPayloadSize(usize, usize),
    ExtraneousHeaderValues,
    InvalidCompressionCode(u16),
    DecompressionFailed,
}

impl TryFrom<Packet> for Command {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        let Packet(Header(command_u16, a, b, c, d), _) = value;
        let command_code = match CommandCode::from_primitive(command_u16) {
            None => {
                return Err(TryFromPacketError::InvalidCommand(command_u16))
            }
            Some(value) => value,
        };

        match command_code {
            CommandCode::Clear => match check_command_only(value) {
                Some(err) => Err(err),
                None => Ok(Command::Clear),
            },
            CommandCode::Brightness => {
                let Packet(header, payload) = value;
                if payload.len() != 1 {
                    return Err(TryFromPacketError::UnexpectedPayloadSize(
                        1,
                        payload.len(),
                    ));
                }

                match check_empty_header(header) {
                    Some(err) => Err(err),
                    None => Ok(Command::Brightness(payload[0])),
                }
            }
            CommandCode::HardReset => match check_command_only(value) {
                Some(err) => Err(err),
                None => Ok(Command::HardReset),
            },
            CommandCode::FadeOut => match check_command_only(value) {
                Some(err) => Err(err),
                None => Ok(Command::FadeOut),
            },
            CommandCode::Cp437Data => {
                let Packet(_, payload) = value;
                Ok(Command::Cp437Data(
                    Origin(a, b),
                    ByteGrid::load(c as usize, d as usize, &payload),
                ))
            }
            CommandCode::CharBrightness => {
                let Packet(_, payload) = value;
                Ok(Command::CharBrightness(
                    Origin(a, b),
                    ByteGrid::load(c as usize, d as usize, &payload),
                ))
            }
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => Ok(Command::BitmapLegacy),
            CommandCode::BitmapLinearWin => {
                let Packet(_, payload) = value;
                Ok(Command::BitmapLinearWin(
                    Origin(a * TILE_SIZE, b),
                    PixelGrid::load(
                        c as usize * TILE_SIZE as usize,
                        d as usize,
                        &payload,
                    ),
                ))
            }
            CommandCode::BitmapLinear => {
                let (vec, compression) = packet_into_linear_bitmap(value)?;
                Ok(Command::BitmapLinear(a, vec, compression))
            }
            CommandCode::BitmapLinearAnd => {
                let (vec, compression) = packet_into_linear_bitmap(value)?;
                Ok(Command::BitmapLinearAnd(a, vec, compression))
            }
            CommandCode::BitmapLinearOr => {
                let (vec, compression) = packet_into_linear_bitmap(value)?;
                Ok(Command::BitmapLinearOr(a, vec, compression))
            }
            CommandCode::BitmapLinearXor => {
                let (vec, compression) = packet_into_linear_bitmap(value)?;
                Ok(Command::BitmapLinearXor(a, vec, compression))
            }
        }
    }
}

fn bitmap_linear_into_packet(
    command: CommandCode,
    offset: Offset,
    compression: CompressionCode,
    payload: Vec<u8>,
) -> Packet {
    let payload = into_compressed(compression, payload);
    let compression = CompressionCode::to_primitive(&compression);
    Packet(
        Header(
            command.to_primitive(),
            offset,
            payload.len() as u16,
            compression,
            0,
        ),
        payload,
    )
}

fn origin_size_payload(
    command: CommandCode,
    origin: Origin,
    size: Size,
    payload: Vec<u8>,
) -> Packet {
    let Origin(x, y) = origin;
    let Size(w, h) = size;
    Packet(Header(command.to_primitive(), x, y, w, h), payload.into())
}

fn command_code_only(code: CommandCode) -> Packet {
    Packet(
        Header(code.to_primitive(), 0x0000, 0x0000, 0x0000, 0x0000),
        vec![],
    )
}

fn check_empty_header(header: Header) -> Option<TryFromPacketError> {
    let Header(_, a, b, c, d) = header;
    if a != 0 || b != 0 || c != 0 || d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

fn check_command_only(packet: Packet) -> Option<TryFromPacketError> {
    let Packet(Header(_, a, b, c, d), payload) = packet;
    if payload.len() != 0 {
        Some(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
    } else if a != 0 || b != 0 || c != 0 || d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

fn packet_into_linear_bitmap(
    packet: Packet,
) -> Result<(BitVec, CompressionCode), TryFromPacketError> {
    let Packet(Header(_, _, length, sub, reserved), payload) = packet;
    if reserved != 0 {
        return Err(TryFromPacketError::ExtraneousHeaderValues);
    }
    if payload.len() != length as usize {
        return Err(TryFromPacketError::UnexpectedPayloadSize(
            length as usize,
            payload.len(),
        ));
    }
    let sub = match CompressionCode::from_primitive(sub) {
        None => return Err(TryFromPacketError::InvalidCompressionCode(sub)),
        Some(value) => value,
    };
    let payload = match into_decompressed(sub, payload) {
        None => return Err(TryFromPacketError::DecompressionFailed),
        Some(value) => value,
    };
    Ok((BitVec::load(&payload), sub))
}
