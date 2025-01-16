//! C functions for interacting with [SPCharGrid]s
//!
//! prefix `sp_char_grid_`

use servicepoint::Grid;
use std::ptr::NonNull;

/// A C-wrapper for grid containing UTF-8 characters.
///
/// As the rust [char] type is not FFI-safe, characters are passed in their UTF-32 form as 32bit unsigned integers.
///
/// The encoding is enforced in most cases by the rust standard library
/// and will panic when provided with illegal characters.
///
/// # Examples
///
/// ```C
/// CharGrid grid = sp_char_grid_new(4, 3);
/// sp_char_grid_fill(grid, '?');
/// sp_char_grid_set(grid, 0, 0, '!');
/// sp_char_grid_free(grid);
/// ```
pub struct SPCharGrid(pub(crate) servicepoint::CharGrid);

impl Clone for SPCharGrid {
    fn clone(&self) -> Self {
        SPCharGrid(self.0.clone())
    }
}

/// Creates a new [SPCharGrid] with the specified dimensions.
///
/// returns: [SPCharGrid] initialized to 0. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_char_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_new(
    width: usize,
    height: usize,
) -> NonNull<SPCharGrid> {
    let result =
        Box::new(SPCharGrid(servicepoint::CharGrid::new(width, height)));
    NonNull::from(Box::leak(result))
}

/// Loads a [SPCharGrid] with the specified dimensions from the provided data.
///
/// Will never return NULL.
///
/// # Panics
///
/// - when `data` is NULL
/// - when the provided `data_length` does not match `height` and `width`
/// - when `data` is not valid UTF-8
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length`
///   bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_char_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> NonNull<SPCharGrid> {
    assert!(data.is_null());
    let data = std::slice::from_raw_parts(data, data_length);
    let result = Box::new(SPCharGrid(
        servicepoint::CharGrid::load_utf8(width, height, data.to_vec())
            .unwrap(),
    ));
    NonNull::from(Box::leak(result))
}

/// Clones a [SPCharGrid].
///
/// Will never return NULL.
///
/// # Panics
///
/// - when `char_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
/// - `char_grid` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_char_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_clone(
    char_grid: *const SPCharGrid,
) -> NonNull<SPCharGrid> {
    assert!(!char_grid.is_null());
    let result = Box::new((*char_grid).clone());
    NonNull::from(Box::leak(result))
}

/// Deallocates a [SPCharGrid].
///
/// # Panics
///
/// - when `char_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
/// - `char_grid` is not used concurrently or after char_grid call
/// - `char_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
///
/// [SPCommand]: [crate::SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_free(char_grid: *mut SPCharGrid) {
    assert!(!char_grid.is_null());
    _ = Box::from_raw(char_grid);
}

/// Gets the current value at the specified position.
///
/// # Arguments
///
/// - `char_grid`: instance to read from
/// - `x` and `y`: position of the cell to read
///
/// # Panics
///
/// - when `char_grid` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
/// - `char_grid` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_get(
    char_grid: *const SPCharGrid,
    x: usize,
    y: usize,
) -> u32 {
    assert!(!char_grid.is_null());
    (*char_grid).0.get(x, y) as u32
}

/// Sets the value of the specified position in the [SPCharGrid].
///
/// # Arguments
///
/// - `char_grid`: instance to write to
/// - `x` and `y`: position of the cell
/// - `value`: the value to write to the cell
///
/// returns: old value of the cell
///
/// # Panics
///
/// - when `char_grid` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPBitVec]
/// - `char_grid` is not written to or read from concurrently
///
/// [SPBitVec]: [crate::SPBitVec]
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_set(
    char_grid: *mut SPCharGrid,
    x: usize,
    y: usize,
    value: u32,
) {
    assert!(!char_grid.is_null());
    (*char_grid).0.set(x, y, char::from_u32(value).unwrap());
}

/// Sets the value of all cells in the [SPCharGrid].
///
/// # Arguments
///
/// - `char_grid`: instance to write to
/// - `value`: the value to set all cells to
///
/// # Panics
///
/// - when `char_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
/// - `char_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_fill(
    char_grid: *mut SPCharGrid,
    value: u32,
) {
    assert!(!char_grid.is_null());
    (*char_grid).0.fill(char::from_u32(value).unwrap());
}

/// Gets the width of the [SPCharGrid] instance.
///
/// # Arguments
///
/// - `char_grid`: instance to read from
///
/// # Panics
///
/// - when `char_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_width(
    char_grid: *const SPCharGrid,
) -> usize {
    assert!(!char_grid.is_null());
    (*char_grid).0.width()
}

/// Gets the height of the [SPCharGrid] instance.
///
/// # Arguments
///
/// - `char_grid`: instance to read from
///
/// # Panics
///
/// - when `char_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `char_grid` points to a valid [SPCharGrid]
#[no_mangle]
pub unsafe extern "C" fn sp_char_grid_height(
    char_grid: *const SPCharGrid,
) -> usize {
    assert!(!char_grid.is_null());
    (*char_grid).0.height()
}
