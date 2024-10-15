//! C functions for interacting with [SPCp437Grid]s
//!
//! prefix `sp_cp437_grid_`

use crate::SPByteSlice;
use servicepoint::{DataRef, Grid};

/// A C-wrapper for grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
///
/// # Examples
///
/// ```C
/// Cp437Grid grid = sp_cp437_grid_new(4, 3);
/// sp_cp437_grid_fill(grid, '?');
/// sp_cp437_grid_set(grid, 0, 0, '!');
/// sp_cp437_grid_free(grid);
/// ```
pub struct SPCp437Grid(pub(crate) servicepoint::Cp437Grid);

impl Clone for SPCp437Grid {
    fn clone(&self) -> Self {
        SPCp437Grid(self.0.clone())
    }
}

/// Creates a new [SPCp437Grid] with the specified dimensions.
///
/// returns: [SPCp437Grid] initialized to 0. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_cp437_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_new(
    width: usize,
    height: usize,
) -> *mut SPCp437Grid {
    let result = Box::into_raw(Box::new(SPCp437Grid(
        servicepoint::Cp437Grid::new(width, height),
    )));
    assert!(!result.is_null());
    result
}

/// Loads a [SPCp437Grid] with the specified dimensions from the provided data.
///
/// Will never return NULL.
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
///   by explicitly calling `sp_cp437_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut SPCp437Grid {
    assert!(data.is_null());
    let data = std::slice::from_raw_parts(data, data_length);
    let result = Box::into_raw(Box::new(SPCp437Grid(
        servicepoint::Cp437Grid::load(width, height, data),
    )));
    assert!(!result.is_null());
    result
}

/// Clones a [SPCp437Grid].
///
/// Will never return NULL.
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
/// - `cp437_grid` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_cp437_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_clone(
    cp437_grid: *const SPCp437Grid,
) -> *mut SPCp437Grid {
    assert!(!cp437_grid.is_null());
    let result = Box::into_raw(Box::new((*cp437_grid).clone()));
    assert!(!result.is_null());
    result
}

/// Deallocates a [SPCp437Grid].
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
/// - `cp437_grid` is not used concurrently or after cp437_grid call
/// - `cp437_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_free(cp437_grid: *mut SPCp437Grid) {
    assert!(!cp437_grid.is_null());
    _ = Box::from_raw(cp437_grid);
}

/// Gets the current value at the specified position.
///
/// # Arguments
///
/// - `cp437_grid`: instance to read from
/// - `x` and `y`: position of the cell to read
///
/// # Panics
///
/// - when `cp437_grid` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
/// - `cp437_grid` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_get(
    cp437_grid: *const SPCp437Grid,
    x: usize,
    y: usize,
) -> u8 {
    assert!(!cp437_grid.is_null());
    (*cp437_grid).0.get(x, y)
}

/// Sets the value of the specified position in the [SPCp437Grid].
///
/// # Arguments
///
/// - `cp437_grid`: instance to write to
/// - `x` and `y`: position of the cell
/// - `value`: the value to write to the cell
///
/// returns: old value of the cell
///
/// # Panics
///
/// - when `cp437_grid` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPBitVec]
/// - `cp437_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_set(
    cp437_grid: *mut SPCp437Grid,
    x: usize,
    y: usize,
    value: u8,
) {
    assert!(!cp437_grid.is_null());
    (*cp437_grid).0.set(x, y, value);
}

/// Sets the value of all cells in the [SPCp437Grid].
///
/// # Arguments
///
/// - `cp437_grid`: instance to write to
/// - `value`: the value to set all cells to
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
/// - `cp437_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_fill(
    cp437_grid: *mut SPCp437Grid,
    value: u8,
) {
    assert!(!cp437_grid.is_null());
    (*cp437_grid).0.fill(value);
}

/// Gets the width of the [SPCp437Grid] instance.
///
/// # Arguments
///
/// - `cp437_grid`: instance to read from
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_width(
    cp437_grid: *const SPCp437Grid,
) -> usize {
    assert!(!cp437_grid.is_null());
    (*cp437_grid).0.width()
}

/// Gets the height of the [SPCp437Grid] instance.
///
/// # Arguments
///
/// - `cp437_grid`: instance to read from
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_height(
    cp437_grid: *const SPCp437Grid,
) -> usize {
    assert!(!cp437_grid.is_null());
    (*cp437_grid).0.height()
}

/// Gets an unsafe reference to the data of the [SPCp437Grid] instance.
///
/// Will never return NULL.
///
/// # Panics
///
/// - when `cp437_grid` is NULL
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `cp437_grid` points to a valid [SPCp437Grid]
/// - the returned memory range is never accessed after the passed [SPCp437Grid] has been freed
/// - the returned memory range is never accessed concurrently, either via the [SPCp437Grid] or directly
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_unsafe_data_ref(
    cp437_grid: *mut SPCp437Grid,
) -> SPByteSlice {
    let data = (*cp437_grid).0.data_ref_mut();
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
