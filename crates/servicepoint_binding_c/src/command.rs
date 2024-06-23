//! C functions for interacting with `Command`s
//!
//! prefix `sp_command_`

use std::ptr::null_mut;

use servicepoint::{
    Brightness, Command, CompressionCode, Offset,
    Origin, Packet, PixelGrid,
};

use crate::bit_vec::CBitVec;
use crate::brightness_grid::CBrightnessGrid;
use crate::cp437_grid::CCp437Grid;

/// Tries to turn a `Packet` into a `Command`. The packet is deallocated in the process.
///
/// Returns: pointer to new `Command` instance or NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `packet` points to a valid instance of `Packet`
/// - `packet` is not used concurrently or after this call
/// - the result is checked for NULL
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
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

/// Clones a `Command` instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid instance of `Command`
/// - `this` is not written to concurrently
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_clone(
    original: *const Command,
) -> *mut Command {
    Box::into_raw(Box::new((*original).clone()))
}

/// Allocates a new `Command::Clear` instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_clear() -> *mut Command {
    Box::into_raw(Box::new(Command::Clear))
}

/// Allocates a new `Command::HardReset` instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_hard_reset() -> *mut Command {
    Box::into_raw(Box::new(Command::HardReset))
}

/// Allocates a new `Command::FadeOut` instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_fade_out() -> *mut Command {
    Box::into_raw(Box::new(Command::FadeOut))
}

/// Allocates a new `Command::Brightness` instance for setting the brightness of all tiles to the
/// same value.
///
/// # Panics
///
/// - When the provided brightness value is out of range (0-11).
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_brightness(brightness: u8) -> *mut Command {
    let brightness =
        Brightness::try_from(brightness).expect("invalid brightness");
    Box::into_raw(Box::new(Command::Brightness(brightness)))
}

/// Allocates a new `Command::CharBrightness` instance.
/// The passed `ByteGrid` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `byte_grid` points to a valid instance of `ByteGrid`
/// - `byte_grid` is not used concurrently or after this call
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_char_brightness(
    x: usize,
    y: usize,
    byte_grid: *mut CBrightnessGrid,
) -> *mut Command {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(Command::CharBrightness(
        Origin::new(x, y),
        byte_grid.0,
    )))
}

/// Allocates a new `Command::BitmapLinear` instance.
/// The passed `BitVec` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of `BitVec`
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear(
    offset: Offset,
    bit_vec: *mut CBitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinear(
        offset,
        bit_vec.into(),
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearAnd` instance.
/// The passed `BitVec` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of `BitVec`
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_and(
    offset: Offset,
    bit_vec: *mut CBitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearAnd(
        offset,
        bit_vec.into(),
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearOr` instance.
/// The passed `BitVec` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of `BitVec`
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_or(
    offset: Offset,
    bit_vec: *mut CBitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearOr(
        offset,
        bit_vec.into(),
        compression,
    )))
}

/// Allocates a new `Command::BitmapLinearXor` instance.
/// The passed `BitVec` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of `BitVec`
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_xor(
    offset: Offset,
    bit_vec: *mut CBitVec,
    compression: CompressionCode,
) -> *mut Command {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(Command::BitmapLinearXor(
        offset,
        bit_vec.into(),
        compression,
    )))
}

/// Allocates a new `Command::Cp437Data` instance.
/// The passed `ByteGrid` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `byte_grid` points to a valid instance of `ByteGrid`
/// - `byte_grid` is not used concurrently or after this call
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_cp437_data(
    x: usize,
    y: usize,
    byte_grid: *mut CCp437Grid,
) -> *mut Command {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(Command::Cp437Data(Origin::new(x, y), byte_grid.0)))
}

/// Allocates a new `Command::BitmapLinearWin` instance.
/// The passed `PixelGrid` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid instance of `PixelGrid`
/// - `pixel_grid` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_win(
    x: usize,
    y: usize,
    pixel_grid: *mut PixelGrid,
    compression_code: CompressionCode,
) -> *mut Command {
    let byte_grid = *Box::from_raw(pixel_grid);
    Box::into_raw(Box::new(Command::BitmapLinearWin(
        Origin::new(x, y),
        byte_grid,
        compression_code,
    )))
}

/// Deallocates a `Command`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Command`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `Packet`
#[no_mangle]
pub unsafe extern "C" fn sp_command_dealloc(ptr: *mut Command) {
    _ = Box::from_raw(ptr);
}
