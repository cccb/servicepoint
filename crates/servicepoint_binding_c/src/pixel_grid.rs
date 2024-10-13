//! C functions for interacting with `SPPixelGrid`s
//!
//! prefix `sp_pixel_grid_`

use servicepoint::{DataRef, Grid};

use crate::byte_slice::SPByteSlice;

/// A grid of pixels.
///
/// # Examples
///
/// ```C
/// Cp437Grid grid = sp_pixel_grid_new(8, 3);
/// sp_pixel_grid_fill(grid, true);
/// sp_pixel_grid_set(grid, 0, 0, false);
/// sp_pixel_grid_free(grid);
/// ```
pub struct SPPixelGrid(pub(crate) servicepoint::PixelGrid);

/// Creates a new `SPPixelGrid` with the specified dimensions.
///
/// # Arguments
///
/// - `width`: size in pixels in x-direction
/// - `height`: size in pixels in y-direction
///
/// returns: `SPPixelGrid` initialized to all pixels off. Will never return NULL.
///
/// # Panics
///
/// - when the width is not dividable by 8
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_pixel_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_new(
    width: usize,
    height: usize,
) -> *mut SPPixelGrid {
    Box::into_raw(Box::new(SPPixelGrid(servicepoint::PixelGrid::new(
        width, height,
    ))))
}

/// Loads a `SPPixelGrid` with the specified dimensions from the provided data.
///
/// # Arguments
///
/// - `width`: size in pixels in x-direction
/// - `height`: size in pixels in y-direction
///
/// returns: `SPPixelGrid` that contains a copy of the provided data. Will never return NULL.
///
/// # Panics
///
/// - when the dimensions and data size do not match exactly.
/// - when the width is not dividable by 8
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length` bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_pixel_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> *mut SPPixelGrid {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(SPPixelGrid(servicepoint::PixelGrid::load(
        width, height, data,
    ))))
}

/// Clones a `SPPixelGrid`.
///
/// Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - `pixel_grid` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_pixel_grid_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_clone(
    pixel_grid: *const SPPixelGrid,
) -> *mut SPPixelGrid {
    Box::into_raw(Box::new(SPPixelGrid((*pixel_grid).0.clone())))
}

/// Deallocates a `SPPixelGrid`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - `pixel_grid` is not used concurrently or after pixel_grid call
/// - `pixel_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_free(pixel_grid: *mut SPPixelGrid) {
    _ = Box::from_raw(pixel_grid);
}

/// Gets the current value at the specified position in the `SPPixelGrid`.
///
/// # Arguments
///
/// - `pixel_grid`: instance to read from
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
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - `pixel_grid` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_get(
    pixel_grid: *const SPPixelGrid,
    x: usize,
    y: usize,
) -> bool {
    (*pixel_grid).0.get(x, y)
}

/// Sets the value of the specified position in the `SPPixelGrid`.
///
/// # Arguments
///
/// - `pixel_grid`: instance to write to
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
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - `pixel_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_set(
    pixel_grid: *mut SPPixelGrid,
    x: usize,
    y: usize,
    value: bool,
) {
    (*pixel_grid).0.set(x, y, value);
}

/// Sets the state of all pixels in the `SPPixelGrid`.
///
/// # Arguments
///
/// - `pixel_grid`: instance to write to
/// - `value`: the value to set all pixels to
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - `pixel_grid` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_fill(
    pixel_grid: *mut SPPixelGrid,
    value: bool,
) {
    (*pixel_grid).0.fill(value);
}

/// Gets the width in pixels of the `SPPixelGrid` instance.
///
/// # Arguments
///
/// - `pixel_grid`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_width(
    pixel_grid: *const SPPixelGrid,
) -> usize {
    (*pixel_grid).0.width()
}

/// Gets the height in pixels of the `SPPixelGrid` instance.
///
/// # Arguments
///
/// - `pixel_grid`: instance to read from
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_height(
    pixel_grid: *const SPPixelGrid,
) -> usize {
    (*pixel_grid).0.height()
}

/// Gets an unsafe reference to the data of the `SPPixelGrid` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `pixel_grid` points to a valid `SPPixelGrid`
/// - the returned memory range is never accessed after the passed `SPPixelGrid` has been freed
/// - the returned memory range is never accessed concurrently, either via the `SPPixelGrid` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_pixel_grid_unsafe_data_ref(
    pixel_grid: *mut SPPixelGrid,
) -> SPByteSlice {
    let data = (*pixel_grid).0.data_ref_mut();
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
