use std::ptr::null_mut;

use servicepoint::{
    BitVec, ByteGrid, CompressionCode, Origin, Packet, PixelGrid,
};
pub use servicepoint::{Brightness, Command, Offset};

/// Tries to turn a `Packet` into a `Command`. The packet is gets deallocated in the process.
///
/// Returns: pointer to command or NULL
#[no_mangle]
pub unsafe extern "C" fn sp_command_try_from_packet(
    packet: *mut Packet,
) -> *mut Command {
    let packet = *Box::from_raw(packet);
    match Command::try_from(packet) {
        Err(_) => null_mut(),
        Ok(command) => Box::into_raw(Box::new(command)),
    }
}

/// Clones a `Command` instance
#[no_mangle]
pub unsafe extern "C" fn sp_command_clone(
    original: *const Command,
) -> *mut Command {
    Box::into_raw(Box::new((*original).clone()))
}

/// Allocates a new `Command::Clear` instance
#[no_mangle]
pub unsafe extern "C" fn sp_command_clear() -> *mut Command {
    Box::into_raw(Box::new(Command::Clear))
}

/// Allocates a new `Command::HardReset` instance
#[no_mangle]
pub unsafe extern "C" fn sp_command_hard_reset() -> *mut Command {
    Box::into_raw(Box::new(Command::HardReset))
}

/// Allocates a new `Command::FadeOut` instance
#[no_mangle]
pub unsafe extern "C" fn sp_command_fade_out() -> *mut Command {
    Box::into_raw(Box::new(Command::FadeOut))
}

/// Allocates a new `Command::Brightness` instance
#[no_mangle]
pub unsafe extern "C" fn sp_command_brightness(
    brightness: Brightness,
) -> *mut Command {
    Box::into_raw(Box::new(Command::Brightness(brightness)))
}

/// Allocates a new `Command::CharBrightness` instance.
/// The passed `ByteGrid` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_char_brightness(
    x: usize,
    y: usize,
    byte_grid: *mut ByteGrid,
) -> *mut Command {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(Command::CharBrightness(Origin(x, y), byte_grid)))
}

/// Allocates a new `Command::BitmapLinear` instance.
/// The passed `BitVec` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear(
    offset: Offset,
    bit_vec: *mut BitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinear(
        offset,
        bit_vec,
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearAnd` instance.
/// The passed `BitVec` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_and(
    offset: Offset,
    bit_vec: *mut BitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearAnd(
        offset,
        bit_vec,
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearOr` instance.
/// The passed `BitVec` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_or(
    offset: Offset,
    bit_vec: *mut BitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearOr(
        offset,
        bit_vec,
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearXor` instance.
/// The passed `BitVec` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_xor(
    offset: Offset,
    bit_vec: *mut BitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearXor(
        offset,
        bit_vec,
        compression,
    )))
}

/// Allocates a new `Command::Cp437Data` instance.
/// The passed `ByteGrid` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_cp437_data(
    x: usize,
    y: usize,
    byte_grid: *mut ByteGrid,
) -> *mut Command {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(Command::Cp437Data(Origin(x, y), byte_grid)))
}

/// Allocates a new `Command::BitmapLinearWin` instance.
/// The passed `PixelGrid` gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_win(
    x: usize,
    y: usize,
    byte_grid: *mut PixelGrid,
    compression_code: CompressionCode,
) -> *mut Command {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(Command::BitmapLinearWin(
        Origin(x, y),
        byte_grid,
        compression_code,
    )))
}

/// Deallocates a `Command`. Note that connection_send does this implicitly, so you only need
/// to do this if you use the library for parsing commands.
#[no_mangle]
pub unsafe extern "C" fn sp_command_dealloc(ptr: *mut Command) {
    _ = Box::from_raw(ptr);
}
