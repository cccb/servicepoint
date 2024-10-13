//! C functions for interacting with [SPCommand]s
//!
//! prefix `sp_command_`

use std::ptr::null_mut;

use servicepoint::{Brightness, Origin};

use crate::{
    SPBitVec, SPBrightnessGrid, SPCompressionCode, SPCp437Grid, SPPacket,
    SPPixelGrid,
};

/// A low-level display command.
///
/// This struct and associated functions implement the UDP protocol for the display.
///
/// To send a [SPCommand], use a `SPConnection`.
///
/// # Examples
///
/// ```C
/// sp_connection_send_command(connection, sp_command_clear());
/// sp_connection_send_command(connection, sp_command_brightness(5));
/// ```
pub struct SPCommand(pub(crate) servicepoint::Command);

impl Clone for SPCommand {
    fn clone(&self) -> Self {
        SPCommand(self.0.clone())
    }
}

/// Tries to turn a `SPPacket` into a [SPCommand].
///
/// The packet is deallocated in the process.
///
/// Returns: pointer to new [SPCommand] instance or NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `SPPacket` points to a valid instance of `SPPacket`
/// - `SPPacket` is not used concurrently or after this call
/// - the result is checked for NULL
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
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

/// Clones a [SPCommand] instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `command` points to a valid instance of [SPCommand]
/// - `command` is not written to concurrently
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_clone(
    command: *const SPCommand,
) -> *mut SPCommand {
    Box::into_raw(Box::new((*command).clone()))
}

/// Set all pixels to the off state.
///
/// Does not affect brightness.
///
/// Returns: a new `Command::Clear` instance. Will never return NULL.
///
/// # Examples
///
/// ```C
/// sp_connection_send_command(connection, sp_command_clear());
/// ```
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_clear() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::Clear)))
}

/// Kills the udp daemon on the display, which usually results in a restart.
///
/// Please do not send this in your normal program flow.
///
/// Returns: a new `Command::HardReset` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_hard_reset() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::HardReset)))
}

/// A yet-to-be-tested command.
///
/// Returns: a new `Command::FadeOut` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_fade_out() -> *mut SPCommand {
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::FadeOut)))
}

/// Set the brightness of all tiles to the same value.
///
/// Returns: a new `Command::Brightness` instance. Will never return NULL.
///
/// # Panics
///
/// - When the provided brightness value is out of range (0-11).
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
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

/// Set the brightness of individual tiles in a rectangular area of the display.
///
/// The passed `SPBrightnessGrid` gets consumed.
///
/// Returns: a new `Command::CharBrightness` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `grid` points to a valid instance of `SPBrightnessGrid`
/// - `grid` is not used concurrently or after this call
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_char_brightness(
    x: usize,
    y: usize,
    grid: *mut SPBrightnessGrid,
) -> *mut SPCommand {
    let byte_grid = *Box::from_raw(grid);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::CharBrightness(
        Origin::new(x, y),
        byte_grid.0,
    ))))
}

/// Set pixel data starting at the pixel offset on screen.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [SPBitVec] is always uncompressed.
///
/// The passed [SPBitVec] gets consumed.
///
/// Returns: a new `Command::BitmapLinear` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of [SPBitVec]
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear(
    offset: usize,
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

/// Set pixel data according to an and-mask starting at the offset.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [SPBitVec] is always uncompressed.
///
/// The passed [SPBitVec] gets consumed.
///
/// Returns: a new `Command::BitmapLinearAnd` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of [SPBitVec]
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_and(
    offset: usize,
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

/// Set pixel data according to an or-mask starting at the offset.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [SPBitVec] is always uncompressed.
///
/// The passed [SPBitVec] gets consumed.
///
/// Returns: a new `Command::BitmapLinearOr` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of [SPBitVec]
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_or(
    offset: usize,
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

/// Set pixel data according to a xor-mask starting at the offset.
///
/// The screen will continuously overwrite more pixel data without regarding the offset, meaning
/// once the starting row is full, overwriting will continue on column 0.
///
/// The contained [SPBitVec] is always uncompressed.
///
/// The passed [SPBitVec] gets consumed.
///
/// Returns: a new `Command::BitmapLinearXor` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid instance of [SPBitVec]
/// - `bit_vec` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_bitmap_linear_xor(
    offset: usize,
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

/// Show text on the screen.
///
/// <div class="warning">
///     The library does not currently convert between UTF-8 and CP-437.
///     Because Rust expects UTF-8 strings, it might be necessary to only send ASCII for now.
/// </div>
///
/// The passed `SPCp437Grid` gets consumed.///
///
/// Returns: a new `Command::Cp437Data` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `grid` points to a valid instance of `SPCp437Grid`
/// - `grid` is not used concurrently or after this call
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_command_cp437_data(
    x: usize,
    y: usize,
    grid: *mut SPCp437Grid,
) -> *mut SPCommand {
    let grid = *Box::from_raw(grid);
    Box::into_raw(Box::new(SPCommand(servicepoint::Command::Cp437Data(
        Origin::new(x, y),
        grid.0,
    ))))
}

/// Sets a window of pixels to the specified values.
///
/// The passed `SPPixelGrid` gets consumed.
///
/// Returns: a new `Command::BitmapLinearWin` instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid instance of `SPPixelGrid`
/// - `pixel_grid` is not used concurrently or after this call
/// - `compression` matches one of the allowed enum values
/// - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_command_free`.
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

/// Deallocates a [SPCommand].
///
/// # Examples
///
/// ```C
/// SPCommand c = sp_command_clear();
/// sp_command_free(c);
/// ```
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `command` points to a valid [SPCommand]
/// - `command` is not used concurrently or after this call
/// - `command` was not passed to another consuming function, e.g. to create a `SPPacket`
#[no_mangle]
pub unsafe extern "C" fn sp_command_free(command: *mut SPCommand) {
    _ = Box::from_raw(command);
}
