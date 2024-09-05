//! C functions for interacting with `Command`s
//!
//! prefix `sp_command_`

use std::ptr::null_mut;

use servicepoint::{Brightness, Origin};

use crate::bit_vec::SPBitVec;
use crate::brightness_grid::SPBrightnessGrid;
use crate::constants::SPCompressionCode;
use crate::cp437_grid::SPCp437Grid;
use crate::packet::SPPacket;
use crate::pixel_grid::SPPixelGrid;
use crate::SPOffset;

/// A low-level display command.
///
/// This struct and associated functions implement the UDP protocol for the display.
///
/// To send a `CCommand`, use a `CConnection`.
///
/// # Examples
///
/// ```C
/// sp_connection_send(connection, sp_command_clear());
/// sp_connection_send(connection, sp_command_brightness(5));
/// ```
pub struct SPCommand(pub(crate) servicepoint::Command);

impl Clone for SPCommand {
    fn clone(&self) -> Self {
        SPCommand(self.0.clone())
    }
}

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
    packet: *mut SPPacket,
) -> *mut SPCommand {
    let packet = *Box::from_raw(packet);
    match servicepoint::Command::try_from(packet.0) {
        Err(_) => null_mut(),
        Ok(command) => Box::into_raw(Box::new(SPCommand(command))),
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
    original: *const SPCommand,
) -> *mut SPCommand {
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
pub unsafe extern "C" fn sp_command_clear() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::Clear)))
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
pub unsafe extern "C" fn sp_command_hard_reset() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::HardReset)))
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
pub unsafe extern "C" fn sp_command_fade_out() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::FadeOut)))
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
/// - the returned `SPCommand` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_brightness(
    brightness: u8,
) -> *mut SPCommand {
    let brightness =
        Brightness::try_from(brightness).expect("invalid brightness");
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::Brightness(
        brightness,
    ))))
}

/// Allocates a new `Command::CharBrightness` instance.
/// The passed `SPBrightnessGrid` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `grid` points to a valid instance of `SPBrightnessGrid`
/// - `grid` is not used concurrently or after this call
/// - the returned `Command` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_char_brightness(
    x: usize,
    y: usize,
    grid: *mut SPBrightnessGrid,
) -> *mut SPCommand {
    let byte_grid = *Box::from_raw(grid);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::CharBrightness(
        Origin::new(x, y),
        byte_grid.actual,
    ))))
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
    offset: SPOffset,
    bit_vec: *mut SPBitVec,
    compression: SPCompressionCode,
) -> *mut SPCommand {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::BitmapLinear(
        offset,
        bit_vec.into(),
        compression.try_into().expect("invalid compression code"),
    ))))
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
    offset: SPOffset,
    bit_vec: *mut SPBitVec,
    compression: SPCompressionCode,
) -> *mut SPCommand {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::BitmapLinearAnd(
        offset,
        bit_vec.into(),
        compression.try_into().expect("invalid compression code"),
    ))))
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
    offset: SPOffset,
    bit_vec: *mut SPBitVec,
    compression: SPCompressionCode,
) -> *mut SPCommand {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::BitmapLinearOr(
        offset,
        bit_vec.into(),
        compression.try_into().expect("invalid compression code"),
    ))))
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
    offset: SPOffset,
    bit_vec: *mut SPBitVec,
    compression: SPCompressionCode,
) -> *mut SPCommand {
    let bit_vec = *Box::from_raw(bit_vec);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::BitmapLinearXor(
        offset,
        bit_vec.into(),
        compression.try_into().expect("invalid compression code"),
    ))))
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
    byte_grid: *mut SPCp437Grid,
) -> *mut SPCommand {
    let byte_grid = *Box::from_raw(byte_grid);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::Cp437Data(
        Origin::new(x, y),
        byte_grid.actual,
    ))))
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
    pixel_grid: *mut SPPixelGrid,
    compression_code: SPCompressionCode,
) -> *mut SPCommand {
    let byte_grid = (*Box::from_raw(pixel_grid)).0;
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::BitmapLinearWin(
        Origin::new(x, y),
        byte_grid,
        compression_code
            .try_into()
            .expect("invalid compression code"),
    ))))
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
pub unsafe extern "C" fn sp_command_dealloc(ptr: *mut SPCommand) {
    _ = Box::from_raw(ptr);
}
