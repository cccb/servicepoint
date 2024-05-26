pub use servicepoint::ByteGrid;
use servicepoint::{DataRef, Grid};

use crate::c_slice::CByteSlice;

/// Creates a new `ByteGrid` instance.
/// The returned instance has to be freed with `byte_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_new(
    width: usize,
    height: usize,
) -> *mut ByteGrid {
    Box::into_raw(Box::new(ByteGrid::new(width, height)))
}

/// Loads a `ByteGrid` with the specified dimensions from the provided data.
/// The returned instance has to be freed with `byte_grid_dealloc`.
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
/// The returned instance has to be freed with `byte_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_clone(
    this: *const ByteGrid,
) -> *mut ByteGrid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `ByteGrid`.
///
/// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_dealloc(this: *mut ByteGrid) {
    _ = Box::from_raw(this);
}

/// Get the current value at the specified position
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_get(
    this: *const ByteGrid,
    x: usize,
    y: usize,
) -> u8 {
    (*this).get(x, y)
}

/// Sets the current value at the specified position
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_set(
    this: *mut ByteGrid,
    x: usize,
    y: usize,
    value: u8,
) {
    (*this).set(x, y, value);
}

/// Fills the whole `ByteGrid` with the specified value
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_fill(this: *mut ByteGrid, value: u8) {
    (*this).fill(value);
}

/// Gets the width in pixels of the `ByteGrid` instance.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_width(this: *const ByteGrid) -> usize {
    (*this).width()
}

/// Gets the height in pixels of the `ByteGrid` instance.
#[no_mangle]
pub unsafe extern "C" fn sp_byte_grid_height(this: *const ByteGrid) -> usize {
    (*this).height()
}

/// Gets an unsafe reference to the data of the `ByteGrid` instance.
///
/// ## Safety
///
/// The caller has to make sure to never access the returned memory after the `ByteGrid`
/// instance has been consumed or manually deallocated.
///
/// Reading and writing concurrently to either the original instance or the returned data will
/// result in undefined behavior.
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
