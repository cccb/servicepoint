mod connection;
mod pixel_grid;
mod bit_vec;

pub use crate::connection::*;
pub use crate::pixel_grid::*;
pub use crate::bit_vec::*;

pub const TILE_SIZE: u16 = 8;
pub const TILE_WIDTH: u16 = 56;
pub const TILE_HEIGHT: u16 = 20;
pub const PIXEL_WIDTH: u16 = TILE_WIDTH * TILE_SIZE;
pub const PIXEL_HEIGHT: u16 = TILE_HEIGHT * TILE_SIZE;

pub const PIXEL_COUNT: usize = PIXEL_WIDTH as usize * PIXEL_HEIGHT as usize;

/// A window
#[derive(Debug)]
pub struct Window(pub Origin, pub Size);

/// An origin marks the top left position of the
/// data sent to the display.
/// A window
#[derive(Debug)]
pub struct Origin(pub u16, pub u16);

/// Size defines the width and height of a window
/// A window
#[derive(Debug)]
pub struct Size(pub u16, pub u16);

type Offset = u16;

type Brightness = u8;

type Packet = Vec<u8>;

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
    BitmapLinearWin(Window, PixelGrid),
}

fn offset_and_payload(command: u16, offset: Offset, payload: Vec<u8>) -> Packet {
    let mut packet = vec!(0u8; 10 + payload.len());

    packet[0..=1].copy_from_slice(&u16::to_be_bytes(command));
    packet[2..=3].copy_from_slice(&u16::to_be_bytes(offset));
    packet[4..=5].copy_from_slice(&u16::to_be_bytes(payload.len() as u16));
    packet[6..=7].copy_from_slice(&[0x00, 0x00]); // subcommand 0 => no compression
    packet[8..=9].copy_from_slice(&[0x00, 0x00]); // reserved

    packet[10..].copy_from_slice(&*payload);

    packet
}

fn window_and_payload(command: u16, window: Window, payload: Vec<u8>) -> Packet {
    let Window(Origin(x, y), Size(w, h)) = window;

    let mut packet = vec!(0u8; 10 + payload.len());
    packet[0..=1].copy_from_slice(&u16::to_be_bytes(command));
    packet[2..=3].copy_from_slice(&u16::to_be_bytes(x));
    packet[4..=5].copy_from_slice(&u16::to_be_bytes(y));
    packet[6..=7].copy_from_slice(&u16::to_be_bytes(w));
    packet[8..=9].copy_from_slice(&u16::to_be_bytes(h));

    packet[10..].copy_from_slice(&*payload);

    packet
}

impl From<Command> for Packet {
    fn from(value: Command) -> Self {
        match value {
            Command::Clear => vec!(0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            Command::CharBrightness(window, payload) => window_and_payload(0x0005, window, payload),
            Command::Brightness(brightness) => vec!(0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, brightness),
            Command::HardReset => vec!(0x00, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            Command::FadeOut => vec!(0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00),
            Command::BitmapLinear(offset, payload) => offset_and_payload(0x0012, offset, payload.into()),
            Command::BitmapLinearWin(window, payload) => window_and_payload(0x0013, window, payload.into()),
            Command::BitmapLinearAnd(offset, payload) => offset_and_payload(0x0014, offset, payload.into()),
            Command::BitmapLinearOr(offset, payload) => offset_and_payload(0x0015, offset, payload.into()),
            Command::BitmapLinearXor(offset, payload) => offset_and_payload(0x0016, offset, payload.into()),
            Command::Cp437Data(window, payload) => window_and_payload(0x0003, window, payload),
        }
    }
}
