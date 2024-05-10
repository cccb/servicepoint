use crate::{BitVec, Packet, PixelGrid, TILE_SIZE};
use crate::command_codes::CommandCode;
use crate::packet::Header;

/// A window
#[derive(Debug, Copy, Clone)]
pub struct Window(pub Origin, pub Size);

/// An origin marks the top left position of the
/// data sent to the display.
#[derive(Debug, Clone, Copy)]
pub struct Origin(pub u16, pub u16);

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
    CharBrightness(Window, Vec<Brightness>),
    Brightness(Brightness),
    #[deprecated]
    BitmapLegacy,
    BitmapLinear(Offset, BitVec),
    BitmapLinearAnd(Offset, BitVec),
    BitmapLinearOr(Offset, BitVec),
    BitmapLinearXor(Offset, BitVec),
    Cp437Data(Window, Vec<u8>),
    BitmapLinearWin(Origin, PixelGrid),
}

fn offset_and_payload(command: CommandCode, offset: Offset, payload: Vec<u8>) -> Packet {
    Packet(Header(command.to_primitive(), offset, payload.len() as u16, 0, 0), payload)
}

fn window_and_payload(command: CommandCode, window: Window, payload: Vec<u8>) -> Packet {
    let Window(Origin(x, y), Size(w, h)) = window;
    Packet(Header(command.to_primitive(), x, y, w, h), payload.into())
}

fn command_code_only(code: CommandCode) -> Packet {
    Packet(Header(code.to_primitive(), 0x0000, 0x0000, 0x0000, 0x0000), vec!())
}

impl Into<Packet> for Command {
    fn into(self) -> Packet {
        match self {
            Command::Clear => command_code_only(CommandCode::Clear),
            Command::FadeOut => command_code_only(CommandCode::FadeOut),
            Command::HardReset => command_code_only(CommandCode::HardReset),
            #[allow(deprecated)]
            Command::BitmapLegacy => command_code_only(CommandCode::BitmapLegacy),

            Command::CharBrightness(window, payload) => {
                window_and_payload(CommandCode::CharBrightness, window, payload)
            }
            Command::Brightness(brightness) => {
                Packet(Header(CommandCode::Brightness.to_primitive(), 0x00000, 0x0000, 0x0000, 0x0000), vec!(brightness))
            }

            Command::BitmapLinear(offset, bits) => {
                offset_and_payload(CommandCode::BitmapLinear, offset, bits.into())
            }
            Command::BitmapLinearWin(Origin(pixel_x, pixel_y), pixels) => {
                debug_assert_eq!(pixel_x % 8, 0);
                debug_assert_eq!(pixels.width % 8, 0);
                Packet(
                    Header(CommandCode::BitmapLinearWin.to_primitive(), pixel_x / TILE_SIZE, pixel_y, pixels.width as u16 / TILE_SIZE, pixels.height as u16),
                    pixels.into())
            }
            Command::BitmapLinearAnd(offset, bits) => {
                offset_and_payload(CommandCode::BitmapLinearAnd, offset, bits.into())
            }
            Command::BitmapLinearOr(offset, bits) => {
                offset_and_payload(CommandCode::BitmapLinearOr, offset, bits.into())
            }
            Command::BitmapLinearXor(offset, bits) => {
                offset_and_payload(CommandCode::BitmapLinearXor, offset, bits.into())
            }
            Command::Cp437Data(window, payload) => {
                window_and_payload(CommandCode::Cp437Data, window, payload)
            }
        }
    }
}

#[derive(Debug)]
pub enum TryFromPacketError {
    InvalidCommand(u16),
    UnexpectedPayloadSize(usize, usize),
    ExtraneousHeaderValues,
    UnsupportedSubcommand(u16),
}

fn check_empty_header(packet: &Packet) -> Option<TryFromPacketError> {
    let Packet(Header(_, a, b, c, d), _) = &packet;
    if *a != 0 || *b != 0 || *c != 0 || *d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

fn check_command_only(packet: &Packet) -> Option<TryFromPacketError> {
    let Packet(_, payload) = packet;
    if payload.len() != 0 {
        Some(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
    } else {
        check_empty_header(packet)
    }
}

fn check_linear_bitmap(packet: &Packet) -> Option<TryFromPacketError> {
    let Packet(Header(_, offset, length, sub, reserved), payload) = packet;
    if *reserved != 0 {
        return Some(TryFromPacketError::ExtraneousHeaderValues);
    }
    if *sub != 0 {
        return Some(TryFromPacketError::UnsupportedSubcommand(*sub));
    }
    if payload.len() != *length as usize {
        return Some(TryFromPacketError::UnexpectedPayloadSize(*length as usize, payload.len()));
    }
    None
}

impl TryFrom<Packet> for Command {
    type Error = TryFromPacketError;

    fn try_from(value: Packet) -> Result<Self, Self::Error> {
        let Packet(Header(command_u16, a, b, c, d), payload) = &value;
        let command_code = match CommandCode::from_primitive(*command_u16) {
            None => return Err(TryFromPacketError::InvalidCommand(*command_u16)),
            Some(value) => value
        };

        match command_code {
            CommandCode::Clear => {
                if let Some(err) = check_command_only(&value) {
                    return Err(err);
                }
                Ok(Command::Clear)
            }
            CommandCode::Brightness => {
                if let Some(err) = check_empty_header(&value) {
                    return Err(err);
                }
                Ok(Command::Brightness(payload[0]))
            }
            CommandCode::HardReset => {
                if let Some(err) = check_command_only(&value) {
                    return Err(err);
                }
                Ok(Command::HardReset)
            }
            CommandCode::FadeOut => {
                if let Some(err) = check_command_only(&value) {
                    return Err(err);
                }
                Ok(Command::FadeOut)
            }
            CommandCode::Cp437Data => {
                Ok(Command::Cp437Data(
                    Window(Origin(*a, *b), Size(*c, *d)),
                    payload.clone(),
                ))
            }
            CommandCode::CharBrightness => {
                Ok(Command::CharBrightness(
                    Window(Origin(*a, *b), Size(*c, *d)),
                    payload.clone(),
                ))
            }
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => {
                Ok(Command::BitmapLegacy)
            }
            CommandCode::BitmapLinearWin => {
                Ok(Command::BitmapLinearWin(
                    Origin(*a * TILE_SIZE, *b),
                    PixelGrid::load(*c as usize * TILE_SIZE as usize, *d as usize, payload),
                ))
            }
            CommandCode::BitmapLinear => {
                if let Some(err) = check_linear_bitmap(&value) {
                    return Err(err);
                }
                Ok(Command::BitmapLinear(*a, BitVec::load(payload)))
            }
            CommandCode::BitmapLinearAnd => {
                if let Some(err) = check_linear_bitmap(&value) {
                    return Err(err);
                }
                Ok(Command::BitmapLinearAnd(*a, BitVec::load(payload)))
            }
            CommandCode::BitmapLinearOr => {
                if let Some(err) = check_linear_bitmap(&value) {
                    return Err(err);
                }
                Ok(Command::BitmapLinearOr(*a, BitVec::load(payload)))
            }
            CommandCode::BitmapLinearXor => {
                if let Some(err) = check_linear_bitmap(&value) {
                    return Err(err);
                }
                Ok(Command::BitmapLinearXor(*a, BitVec::load(payload)))
            }
        }
    }
}
