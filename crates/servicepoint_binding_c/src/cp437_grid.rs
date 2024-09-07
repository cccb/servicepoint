//! C functions for interacting with `SPCp437Grid`s
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
pub struct SPCp437Grid {
    pub(crate) actual: servicepoint::Cp437Grid,
}

impl Clone for SPCp437Grid {
    fn clone(&self) -> Self {
        SPCp437Grid {
            actual: self.actual.clone(),
        }
    }
}

/// Creates a new `SPCp437Grid` with the specified dimensions.
///
/// returns: `SPCp437Grid` initialized to 0.
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
    Box::into_raw(Box::new(SPCp437Grid {
        actual: servicepoint::Cp437Grid::new(width, height),
    }))
}

/// Loads a `SPCp437Grid` with the specified dimensions from the provided data.
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
///   by explicitly calling `sp_cp437_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut SPCp437Grid {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(SPCp437Grid {
        actual: servicepoint::Cp437Grid::load(width, height, data),
    }))
}

/// Clones a `SPCp437Grid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_cp437_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_clone(
    this: *const SPCp437Grid,
) -> *mut SPCp437Grid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `SPCp437Grid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `SPCommand`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_free(this: *mut SPCp437Grid) {
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
/// - `this` points to a valid `SPCp437Grid`
/// - `this` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_get(
    this: *const SPCp437Grid,
    x: usize,
    y: usize,
) -> u8 {
    (*this).actual.get(x, y)
}

/// Sets the value of the specified position in the `SPCp437Grid`.
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
/// When accessing `x` or `y` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPBitVec`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_set(
    this: *mut SPCp437Grid,
    x: usize,
    y: usize,
    value: u8,
) {
    (*this).actual.set(x, y, value);
}

/// Sets the value of all cells in the `SPCp437Grid`.
///
/// # Arguments
///
/// - `this`: instance to write to
/// - `value`: the value to set all cells to
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_fill(this: *mut SPCp437Grid, value: u8) {
    (*this).actual.fill(value);
}

/// Gets the width of the `SPCp437Grid` instance.
///
/// # Arguments
///
/// - `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_width(
    this: *const SPCp437Grid,
) -> usize {
    (*this).actual.width()
}

/// Gets the height of the `SPCp437Grid` instance.
///
/// # Arguments
///
/// - `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_height(
    this: *const SPCp437Grid,
) -> usize {
    (*this).actual.height()
}

/// Gets an unsafe reference to the data of the `SPCp437Grid` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPCp437Grid`
/// - the returned memory range is never accessed after the passed `SPCp437Grid` has been freed
/// - the returned memory range is never accessed concurrently, either via the `SPCp437Grid` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_unsafe_data_ref(
    this: *mut SPCp437Grid,
) -> SPByteSlice {
    let data = (*this).actual.data_ref_mut();
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
