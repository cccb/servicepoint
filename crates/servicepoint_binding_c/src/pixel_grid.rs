use servicepoint::{DataRef, Grid, PixelGrid};

use crate::c_slice::CByteSlice;

/// Creates a new `PixelGrid` instance.
/// The returned instance has to be freed with `pixel_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_new(
    width: usize,
    height: usize,
) -> *mut PixelGrid {
    Box::into_raw(Box::new(PixelGrid::new(width, height)))
}

/// Loads a `PixelGrid` with the specified dimensions from the provided data.
/// The returned instance has to be freed with `pixel_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut PixelGrid {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(PixelGrid::load(width, height, data)))
}

/// Clones a `PixelGrid`.
/// The returned instance has to be freed with `pixel_grid_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_clone(
    this: *const PixelGrid,
) -> *mut PixelGrid {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `PixelGrid`.
///
/// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_dealloc(this: *mut PixelGrid) {
    _ = Box::from_raw(this);
}

/// Get the current value at the specified position
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_get(
    this: *const PixelGrid,
    x: usize,
    y: usize,
) -> bool {
    (*this).get(x, y)
}

/// Sets the current value at the specified position
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_set(
    this: *mut PixelGrid,
    x: usize,
    y: usize,
    value: bool,
) {
    (*this).set(x, y, value);
}

/// Fills the whole `PixelGrid` with the specified value
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_fill(this: *mut PixelGrid, value: bool) {
    (*this).fill(value);
}

/// Gets the width in pixels of the `PixelGrid` instance.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_width(this: *const PixelGrid) -> usize {
    (*this).width()
}

/// Gets the height in pixels of the `PixelGrid` instance.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_height(this: *const PixelGrid) -> usize {
    (*this).height()
}

/// Gets an unsafe reference to the data of the `PixelGrid` instance.
///
/// ## Safety
///
/// The caller has to make sure to never access the returned memory after the `PixelGrid`
/// instance has been consumed or manually deallocated.
///
/// Reading and writing concurrently to either the original instance or the returned data will
/// result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_unsafe_data_ref(
    this: *mut PixelGrid,
) -> CByteSlice {
    let data = (*this).data_ref_mut();
    CByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
