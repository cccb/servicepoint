//! C functions for interacting with [SPBitmap]s
//!
//! prefix `sp_bitmap_`

use std::ptr::NonNull;
use servicepoint::{DataRef, Grid};

use crate::byte_slice::SPByteSlice;

/// A grid of pixels.
///
/// # Examples
///
/// ```C
/// Cp437Grid grid = sp_bitmap_new(8, 3);
/// sp_bitmap_fill(grid, true);
/// sp_bitmap_set(grid, 0, 0, false);
/// sp_bitmap_free(grid);
/// ```
pub struct SPBitmap(pub(crate) servicepoint::Bitmap);

/// Creates a new [SPBitmap] with the specified dimensions.
///
/// # Arguments
///
/// - `width`: size in pixels in x-direction
/// - `height`: size in pixels in y-direction
///
/// returns: [SPBitmap] initialized to all pixels off. Will never return NULL.
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
///   by explicitly calling `sp_bitmap_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_new(
    width: usize,
    height: usize,
) -> NonNull<SPBitmap> {
    let result = Box::new(SPBitmap(servicepoint::Bitmap::new(
        width, height,
    )));
    NonNull::from(Box::leak(result))
}

/// Creates a new [SPBitmap] with a size matching the screen.
///
/// returns: [SPBitmap] initialized to all pixels off. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling [sp_bitmap_free].
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_new_screen_sized() -> NonNull<SPBitmap> {
    let result = Box::new(SPBitmap(servicepoint::Bitmap::max_sized()));
    NonNull::from(Box::leak(result))
}

/// Loads a [SPBitmap] with the specified dimensions from the provided data.
///
/// # Arguments
///
/// - `width`: size in pixels in x-direction
/// - `height`: size in pixels in y-direction
///
/// returns: [SPBitmap] that contains a copy of the provided data. Will never return NULL.
///
/// # Panics
///
/// - when `data` is NULL
/// - when the dimensions and data size do not match exactly.
/// - when the width is not dividable by 8
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length` bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bitmap_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_load(
    width: usize,
    height: usize,
    data: *const u8,
    data_length: usize,
) -> NonNull<SPBitmap> {
    assert!(!data.is_null());
    let data = std::slice::from_raw_parts(data, data_length);
    let result = Box::new(SPBitmap(servicepoint::Bitmap::load(
        width, height, data,
    )));
    NonNull::from(Box::leak(result))
}

/// Clones a [SPBitmap].
///
/// Will never return NULL.
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - `bitmap` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bitmap_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_clone(
    bitmap: *const SPBitmap,
) -> NonNull<SPBitmap> {
    assert!(!bitmap.is_null());
    let result = Box::new(SPBitmap((*bitmap).0.clone()));
    NonNull::from(Box::leak(result))
}

/// Deallocates a [SPBitmap].
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - `bitmap` is not used concurrently or after bitmap call
/// - `bitmap` was not passed to another consuming function, e.g. to create a [SPCommand]
///
/// [SPCommand]: [crate::SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_free(bitmap: *mut SPBitmap) {
    assert!(!bitmap.is_null());
    _ = Box::from_raw(bitmap);
}

/// Gets the current value at the specified position in the [SPBitmap].
///
/// # Arguments
///
/// - `bitmap`: instance to read from
/// - `x` and `y`: position of the cell to read
///
/// # Panics
///
/// - when `bitmap` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - `bitmap` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_get(
    bitmap: *const SPBitmap,
    x: usize,
    y: usize,
) -> bool {
    assert!(!bitmap.is_null());
    (*bitmap).0.get(x, y)
}

/// Sets the value of the specified position in the [SPBitmap].
///
/// # Arguments
///
/// - `bitmap`: instance to write to
/// - `x` and `y`: position of the cell
/// - `value`: the value to write to the cell
///
/// returns: old value of the cell
///
/// # Panics
///
/// - when `bitmap` is NULL
/// - when accessing `x` or `y` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - `bitmap` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_set(
    bitmap: *mut SPBitmap,
    x: usize,
    y: usize,
    value: bool,
) {
    assert!(!bitmap.is_null());
    (*bitmap).0.set(x, y, value);
}

/// Sets the state of all pixels in the [SPBitmap].
///
/// # Arguments
///
/// - `bitmap`: instance to write to
/// - `value`: the value to set all pixels to
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - `bitmap` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_fill(bitmap: *mut SPBitmap, value: bool) {
    assert!(!bitmap.is_null());
    (*bitmap).0.fill(value);
}

/// Gets the width in pixels of the [SPBitmap] instance.
///
/// # Arguments
///
/// - `bitmap`: instance to read from
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_width(bitmap: *const SPBitmap) -> usize {
    assert!(!bitmap.is_null());
    (*bitmap).0.width()
}

/// Gets the height in pixels of the [SPBitmap] instance.
///
/// # Arguments
///
/// - `bitmap`: instance to read from
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_height(bitmap: *const SPBitmap) -> usize {
    assert!(!bitmap.is_null());
    (*bitmap).0.height()
}

/// Gets an unsafe reference to the data of the [SPBitmap] instance.
///
/// # Panics
///
/// - when `bitmap` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bitmap` points to a valid [SPBitmap]
/// - the returned memory range is never accessed after the passed [SPBitmap] has been freed
/// - the returned memory range is never accessed concurrently, either via the [SPBitmap] or directly
#[no_mangle]
pub unsafe extern "C" fn sp_bitmap_unsafe_data_ref(
    bitmap: *mut SPBitmap,
) -> SPByteSlice {
    assert!(!bitmap.is_null());
    let data = (*bitmap).0.data_ref_mut();
    SPByteSlice {
        start: NonNull::new(data.as_mut_ptr_range().start).unwrap(),
        length: data.len(),
    }
}
