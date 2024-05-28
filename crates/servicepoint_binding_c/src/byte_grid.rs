pub use servicepoint::ByteGrid;
use servicepoint::{DataRef, Grid};

use crate::c_slice::CByteSlice;

/// Creates a new `ByteGrid` with the specified dimensions.
///
/// returns: `ByteGrid` initialized to 0.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_byte_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_new(
    width: usize,
    height: usize,
) -> *mut ByteGrid {
    Box::into_raw(Box::new(ByteGrid::new(width, height)))
}

/// Loads a `ByteGrid` with the specified dimensions from the provided data.
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
///   by explicitly calling `sp_byte_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut ByteGrid {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(ByteGrid::load(width, height, data)))
}

/// Clones a `ByteGrid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `ByteGrid`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_byte_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_clone(
    this: *const ByteGrid,
) -> *mut ByteGrid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `ByteGrid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `ByteGrid`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `Command`
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_dealloc(this: *mut ByteGrid) {
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
/// - `this` points to a valid `ByteGrid`
/// - `this` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_get(
    this: *const ByteGrid,
    x: usize,
    y: usize,
) -> u8 {
    (*this).get(x, y)
}

/// Sets the value of the specified position in the `ByteGrid`.
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
pub unsafe extern "C" fn sp_byte_grid_set(
    this: *mut ByteGrid,
    x: usize,
    y: usize,
    value: u8,
) {
    (*this).set(x, y, value);
}

/// Sets the value of all cells in the `ByteGrid`.
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
/// - `this` points to a valid `ByteGrid`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_fill(this: *mut ByteGrid, value: u8) {
    (*this).fill(value);
}

/// Gets the width in pixels of the `ByteGrid` instance.
///
/// # Arguments
///
/// * `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `ByteGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_width(this: *const ByteGrid) -> usize {
    (*this).width()
}

/// Gets the height in pixels of the `ByteGrid` instance.
///
/// # Arguments
///
/// * `this`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `ByteGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_height(this: *const ByteGrid) -> usize {
    (*this).height()
}

/// Gets an unsafe reference to the data of the `ByteGrid` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `ByteGrid`
/// - the returned memory range is never accessed after the passed `ByteGrid` has been freed
/// - the returned memory range is never accessed concurrently, either via the `ByteGrid` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_unsafe_data_ref(
    this: *mut ByteGrid,
) -> CByteSlice {
    let data = (*this).data_ref_mut();
    CByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
