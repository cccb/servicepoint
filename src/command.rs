use crate::{BitVec, Header, Packet, PixelGrid, TILE_SIZE, ToPacket};
use crate::command_codes::DisplayCommandCode;

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
    BitmapLinear(Offset, BitVec),
    BitmapLinearAnd(Offset, BitVec),
    BitmapLinearOr(Offset, BitVec),
    BitmapLinearXor(Offset, BitVec),
    Cp437Data(Window, Vec<u8>),
    BitmapLinearWin(Origin, PixelGrid),
}

fn offset_and_payload(command: DisplayCommandCode, offset: Offset, payload: Vec<u8>) -> Packet {
    Packet(Header(command.to_primitive(), offset, payload.len() as u16, 0, 0), payload)
}

fn window_and_payload(command: DisplayCommandCode, window: Window, payload: Vec<u8>) -> Packet {
    let Window(Origin(x, y), Size(w, h)) = window;
    Packet(Header(command.to_primitive(), x, y, w, h), payload.into())
}

fn command_code_only(code: DisplayCommandCode) -> Packet {
    Packet(Header(code.to_primitive(), 0x0000, 0x0000, 0x0000, 0x0000), vec!())
}

impl ToPacket for Command {
    fn to_packet(self) -> Packet {
        match self {
            Command::Clear => command_code_only(DisplayCommandCode::Clear),
            Command::FadeOut => command_code_only(DisplayCommandCode::FadeOut),
            Command::HardReset => command_code_only(DisplayCommandCode::HardReset),

            Command::CharBrightness(window, payload) => {
                window_and_payload(DisplayCommandCode::CharBrightness, window, payload)
            }
            Command::Brightness(brightness) => {
                Packet(Header(DisplayCommandCode::Brightness.to_primitive(), 0x00000, 0x0000, 0x0000, 0x0000), vec!(brightness))
            }

            Command::BitmapLinear(offset, bits) => {
                offset_and_payload(DisplayCommandCode::BitmapLinear, offset, bits.into())
            }
            Command::BitmapLinearWin(Origin(pixel_x, pixel_y), pixels) => {
                debug_assert_eq!(pixel_x % 8, 0);
                debug_assert_eq!(pixels.width % 8, 0);
                Packet(Header(0x0013, pixel_x / TILE_SIZE, pixel_y, pixels.width as u16 / TILE_SIZE, pixels.height as u16), pixels.into())
            }
            Command::BitmapLinearAnd(offset, bits) => {
                offset_and_payload(DisplayCommandCode::BitmapLinearAnd, offset, bits.into())
            }
            Command::BitmapLinearOr(offset, bits) => {
                offset_and_payload(DisplayCommandCode::BitmapLinearOr, offset, bits.into())
            }
            Command::BitmapLinearXor(offset, bits) => {
                offset_and_payload(DisplayCommandCode::BitmapLinearXor, offset, bits.into())
            }
            Command::Cp437Data(window, payload) => {
                window_and_payload(DisplayCommandCode::Cp437data, window, payload)
            }
        }
    }
}
