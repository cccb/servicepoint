//! C functions for interacting with [SPBrightnessGrid]s
//!
//! prefix `sp_brightness_grid_`

use crate::SPByteSlice;
use servicepoint::{DataRef, Grid};
use std::convert::Into;
use std::intrinsics::transmute;
use std::ptr::NonNull;

/// see [servicepoint::Brightness::MIN]
pub const SP_BRIGHTNESS_MIN: u8 = 0;
/// see [servicepoint::Brightness::MAX]
pub const SP_BRIGHTNESS_MAX: u8 = 11;
/// Count of possible brightness values
pub const SP_BRIGHTNESS_LEVELS: u8 = 12;

/// A grid containing brightness values.
///
/// # Examples
/// ```C
/// SPConnection connection = sp_connection_open("127.0.0.1:2342");
/// if (connection == NULL)
///     return 1;
///
/// SPBrightnessGrid grid = sp_brightness_grid_new(2, 2);
/// sp_brightness_grid_set(grid, 0, 0, 0);
/// sp_brightness_grid_set(grid, 1, 1, 10);
///
/// SPCommand command = sp_command_char_brightness(grid);
/// sp_connection_free(connection);
/// ```
#[derive(Clone)]
pub struct SPBrightnessGrid(pub(crate) servicepoint::BrightnessGrid);

/// Creates a new [SPBrightnessGrid] with the specified dimensions.
///
/// returns: [SPBrightnessGrid] initialized to 0. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_new(
    width: usize,
    height: usize,
) -> NonNull<SPBrightnessGrid> {
    let result = Box::new(SPBrightnessGrid(servicepoint::BrightnessGrid::new(
        width, height,
    )));
    NonNull::from(Box::leak(result))
}

/// Loads a [SPBrightnessGrid] with the specified dimensions from the provided data.
///
/// returns: new [SPBrightnessGrid] instance. Will never return NULL.
///
/// # Panics
///
/// - when `data` is NULL
/// - when the provided `data_length` does not match `height` and `width`
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length`
///   bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> NonNull<SPBrightnessGrid> {
    assert!(!data.is_null());
    let data = std::slice::from_raw_parts(data, data_length);
    let grid = servicepoint::ByteGrid::load(width, height, data);
    let grid = servicepoint::BrightnessGrid::try_from(grid)
        .expect("invalid brightness value");
    let result = Box::new(SPBrightnessGrid(grid));
    NonNull::from(Box::leak(result))
}

/// Clones a [SPBrightnessGrid].
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
///
/// returns: new [SPBrightnessGrid] instance. Will never return NULL.
///
/// # Panics
///
/// - when `brightness_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - `brightness_grid` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_clone(
    brightness_grid: *const SPBrightnessGrid,
) -> NonNull<SPBrightnessGrid> {
    assert!(!brightness_grid.is_null());
    let result = Box::new((*brightness_grid).clone());
    NonNull::from(Box::leak(result))
}

/// Deallocates a [SPBrightnessGrid].
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
///
/// # Panics
///
/// - when `brightness_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - `brightness_grid` is not used concurrently or after this call
/// - `brightness_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
///
/// [SPCommand]: [crate::SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_free(
    brightness_grid: *mut SPBrightnessGrid,
) {
    assert!(!brightness_grid.is_null());
    _ = Box::from_raw(brightness_grid);
}

/// Gets the current value at the specified position.
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
/// - `x` and `y`: position of the cell to read
///
/// returns: value at position
///
/// # Panics
///
/// - when `brightness_grid` is NULL
/// - When accessing `x` or `y` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - `brightness_grid` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_get(
    brightness_grid: *const SPBrightnessGrid,
    x: usize,
    y: usize,
) -> u8 {
    assert!(!brightness_grid.is_null());
    (*brightness_grid).0.get(x, y).into()
}

/// Sets the value of the specified position in the [SPBrightnessGrid].
///
/// # Arguments
///
/// - `brightness_grid`: instance to write to
/// - `x` and `y`: position of the cell
/// - `value`: the value to write to the cell
///
/// returns: old value of the cell
///
/// # Panics
///
/// - when `brightness_grid` is NULL
/// - When accessing `x` or `y` out of bounds.
/// - When providing an invalid brightness value
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - `brightness_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_set(
    brightness_grid: *mut SPBrightnessGrid,
    x: usize,
    y: usize,
    value: u8,
) {
    assert!(!brightness_grid.is_null());
    let brightness = servicepoint::Brightness::try_from(value)
        .expect("invalid brightness value");
    (*brightness_grid).0.set(x, y, brightness);
}

/// Sets the value of all cells in the [SPBrightnessGrid].
///
/// # Arguments
///
/// - `brightness_grid`: instance to write to
/// - `value`: the value to set all cells to
///
/// # Panics
///
/// - when `brightness_grid` is NULL
/// - When providing an invalid brightness value
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - `brightness_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_fill(
    brightness_grid: *mut SPBrightnessGrid,
    value: u8,
) {
    assert!(!brightness_grid.is_null());
    let brightness = servicepoint::Brightness::try_from(value)
        .expect("invalid brightness value");
    (*brightness_grid).0.fill(brightness);
}

/// Gets the width of the [SPBrightnessGrid] instance.
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
///
/// returns: width
///
/// # Panics
///
/// - when `brightness_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_width(
    brightness_grid: *const SPBrightnessGrid,
) -> usize {
    assert!(!brightness_grid.is_null());
    (*brightness_grid).0.width()
}

/// Gets the height of the [SPBrightnessGrid] instance.
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
///
/// returns: height
///
/// # Panics
///
/// - when `brightness_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_height(
    brightness_grid: *const SPBrightnessGrid,
) -> usize {
    assert!(!brightness_grid.is_null());
    (*brightness_grid).0.height()
}

/// Gets an unsafe reference to the data of the [SPBrightnessGrid] instance.
///
/// # Arguments
///
/// - `brightness_grid`: instance to read from
///
/// returns: slice of bytes underlying the `brightness_grid`.
///
/// # Panics
///
/// - when `brightness_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `brightness_grid` points to a valid [SPBrightnessGrid]
/// - the returned memory range is never accessed after the passed [SPBrightnessGrid] has been freed
/// - the returned memory range is never accessed concurrently, either via the [SPBrightnessGrid] or directly
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_unsafe_data_ref(
    brightness_grid: *mut SPBrightnessGrid,
) -> SPByteSlice {
    assert!(!brightness_grid.is_null());
    assert_eq!(core::mem::size_of::<servicepoint::Brightness>(), 1);
    let data = (*brightness_grid).0.data_ref_mut();
    // this assumes more about the memory layout than rust guarantees. yikes!
    let data: &mut [u8] = transmute(data);
    SPByteSlice {
        start: NonNull::new(data.as_mut_ptr_range().start).unwrap(),
        length: data.len(),
    }
}
