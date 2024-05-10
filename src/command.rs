use crate::{BitVec, Header, Packet, PixelGrid, TILE_SIZE, ToPacket};

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

fn offset_and_payload(command: u16, offset: Offset, payload: Vec<u8>) -> Packet {
    Packet(Header(command, offset, payload.len() as u16, 0, 0), payload)
}

fn window_and_payload(command: u16, window: Window, payload: Vec<u8>) -> Packet {
    let Window(Origin(x, y), Size(w, h)) = window;
    Packet(Header(command, x, y, w, h), payload.into())
}

impl ToPacket for Command {
    fn to_packet(self) -> Packet {
        match self {
            Command::Clear => Packet(Header(0x0002, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::CharBrightness(window, payload) => window_and_payload(0x0005, window, payload),
            Command::Brightness(brightness) => Packet(Header(0x0007, 0x00000, 0x0000, 0x0000, 0x0000), vec!(brightness)),
            Command::HardReset => Packet(Header(0x000b, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::FadeOut => Packet(Header(0x000d, 0x0000, 0x0000, 0x0000, 0x0000), vec!()),
            Command::BitmapLinear(offset, bits) => offset_and_payload(0x0012, offset, bits.into()),
            Command::BitmapLinearWin(Origin(pixel_x, pixel_y), pixels) => {
                debug_assert_eq!(pixel_x % 8, 0);
                debug_assert_eq!(pixels.width % 8, 0);
                Packet(Header(0x0013, pixel_x / TILE_SIZE, pixel_y, pixels.width as u16/ TILE_SIZE,pixels.height as u16), pixels.into())
            }
            Command::BitmapLinearAnd(offset, bits) => offset_and_payload(0x0014, offset, bits.into()),
            Command::BitmapLinearOr(offset, bits) => offset_and_payload(0x0015, offset, bits.into()),
            Command::BitmapLinearXor(offset, bits) => offset_and_payload(0x0016, offset, bits.into()),
            Command::Cp437Data(window, payload) => window_and_payload(0x0003, window, payload),
        }
    }
}
