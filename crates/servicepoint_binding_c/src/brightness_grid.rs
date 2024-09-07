//! C functions for interacting with `BrightnessGrid`s
//!
//! prefix `sp_brightness_grid_`

use crate::SPByteSlice;
use servicepoint::{Brightness, DataRef, Grid, PrimitiveGrid};
use std::intrinsics::transmute;

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
/// sp_connection_dealloc(connection);
/// ```
pub struct SPBrightnessGrid {
    pub(crate) actual: servicepoint::BrightnessGrid,
}

impl Clone for SPBrightnessGrid {
    fn clone(&self) -> Self {
        SPBrightnessGrid {
            actual: self.actual.clone(),
        }
    }
}

/// Creates a new `BrightnessGrid` with the specified dimensions.
///
/// returns: `BrightnessGrid` initialized to 0.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_new(
    width: usize,
    height: usize,
) -> *mut SPBrightnessGrid {
    Box::into_raw(Box::new(SPBrightnessGrid {
        actual: servicepoint::BrightnessGrid::new(width, height),
    }))
}

/// Loads a `BrightnessGrid` with the specified dimensions from the provided data.
///
/// # Panics
///
/// When the provided `data_length` is not sufficient for the `height` and `width`
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length`
///   bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut SPBrightnessGrid {
    let data = std::slice::from_raw_parts(data, data_length);
    let grid = PrimitiveGrid::load(width, height, data);
    let grid = servicepoint::BrightnessGrid::try_from(grid)
        .expect("invalid brightness value");
    Box::into_raw(Box::new(SPBrightnessGrid { actual: grid }))
}

/// Clones a `BrightnessGrid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_brightness_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_clone(
    this: *const SPBrightnessGrid,
) -> *mut SPBrightnessGrid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `BrightnessGrid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `Command`
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_dealloc(
    this: *mut SPBrightnessGrid,
) {
    _ = Box::from_raw(this);
}

/// Gets the current value at the specified position.
///
/// # Arguments
///
/// - `this`: instance to read from
/// - `x` and `y`: position of the cell to read
///
/// # Panics
///
/// When accessing `x` or `y` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
/// - `this` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_get(
    this: *const SPBrightnessGrid,
    x: usize,
    y: usize,
) -> u8 {
    (*this).actual.get(x, y).into()
}

/// Sets the value of the specified position in the `BrightnessGrid`.
///
/// # Arguments
///
/// - `this`: instance to write to
/// - `x` and `y`: position of the cell
/// - `value`: the value to write to the cell
///
/// returns: old value of the cell
///
/// # Panics
///
/// - When accessing `x` or `y` out of bounds.
/// - When providing an invalid brightness value
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_set(
    this: *mut SPBrightnessGrid,
    x: usize,
    y: usize,
    value: u8,
) {
    let brightness =
        Brightness::try_from(value).expect("invalid brightness value");
    (*this).actual.set(x, y, brightness);
}

/// Sets the value of all cells in the `BrightnessGrid`.
///
/// # Arguments
///
/// - `this`: instance to write to
/// - `value`: the value to set all cells to
///
/// # Panics
///
/// - When providing an invalid brightness value
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_fill(
    this: *mut SPBrightnessGrid,
    value: u8,
) {
    let brightness =
        Brightness::try_from(value).expect("invalid brightness value");
    (*this).actual.fill(brightness);
}

/// Gets the width of the `BrightnessGrid` instance.
///
/// # Arguments
///
/// - `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_width(
    this: *const SPBrightnessGrid,
) -> usize {
    (*this).actual.width()
}

/// Gets the height of the `BrightnessGrid` instance.
///
/// # Arguments
///
/// - `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_height(
    this: *const SPBrightnessGrid,
) -> usize {
    (*this).actual.height()
}

/// Gets an unsafe reference to the data of the `BrightnessGrid` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BrightnessGrid`
/// - the returned memory range is never accessed after the passed `BrightnessGrid` has been freed
/// - the returned memory range is never accessed concurrently, either via the `BrightnessGrid` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_brightness_grid_unsafe_data_ref(
    this: *mut SPBrightnessGrid,
) -> SPByteSlice {
    assert_eq!(std::mem::size_of::<Brightness>(), 1);

    let data = (*this).actual.data_ref_mut();
    let data: &mut [u8] = transmute(data);
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
