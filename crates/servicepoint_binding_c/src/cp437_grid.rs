//! C functions for interacting with `Cp437Grid`s
//!
//! prefix `sp_cp437_grid_`

use servicepoint::{Cp437Grid, DataRef, Grid};

use crate::c_slice::CByteSlice;

/// A C-wrapper for grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
#[derive(Clone)]
pub struct CCp437Grid(pub(crate) Cp437Grid);

/// Creates a new `Cp437Grid` with the specified dimensions.
///
/// returns: `Cp437Grid` initialized to 0.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_cp437_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_new(
    width: usize,
    height: usize,
) -> *mut CCp437Grid {
    Box::into_raw(Box::new(CCp437Grid(Cp437Grid::new(width, height))))
}

/// Loads a `Cp437Grid` with the specified dimensions from the provided data.
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
///   by explicitly calling `sp_cp437_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut CCp437Grid {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(CCp437Grid(Cp437Grid::load(width, height, data))))
}

/// Clones a `Cp437Grid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_cp437_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_clone(
    this: *const CCp437Grid,
) -> *mut CCp437Grid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `Cp437Grid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `Command`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_dealloc(this: *mut CCp437Grid) {
    _ = Box::from_raw(this);
}

/// Gets the current value at the specified position.
///
/// # Arguments
///
/// * `this`: instance to read from
/// * `x` and `y`: position of the cell to read
///
/// # Panics
///
/// When accessing `x` or `y` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
/// - `this` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_get(
    this: *const CCp437Grid,
    x: usize,
    y: usize,
) -> u8 {
    (*this).0.get(x, y)
}

/// Sets the value of the specified position in the `Cp437Grid`.
///
/// # Arguments
///
/// * `this`: instance to write to
/// * `x` and `y`: position of the cell
/// * `value`: the value to write to the cell
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
/// - `this` points to a valid `BitVec`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_set(
    this: *mut CCp437Grid,
    x: usize,
    y: usize,
    value: u8,
) {
    (*this).0.set(x, y, value);
}

/// Sets the value of all cells in the `Cp437Grid`.
///
/// # Arguments
///
/// * `this`: instance to write to
/// * `value`: the value to set all cells to
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_fill(this: *mut CCp437Grid, value: u8) {
    (*this).0.fill(value);
}

/// Gets the width of the `Cp437Grid` instance.
///
/// # Arguments
///
/// * `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_width(this: *const CCp437Grid) -> usize {
    (*this).0.width()
}

/// Gets the height of the `Cp437Grid` instance.
///
/// # Arguments
///
/// * `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_height(this: *const CCp437Grid) -> usize {
    (*this).0.height()
}

/// Gets an unsafe reference to the data of the `Cp437Grid` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Cp437Grid`
/// - the returned memory range is never accessed after the passed `Cp437Grid` has been freed
/// - the returned memory range is never accessed concurrently, either via the `Cp437Grid` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_cp437_grid_unsafe_data_ref(
    this: *mut CCp437Grid,
) -> CByteSlice {
    let data = (*this).0.data_ref_mut();
    CByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
