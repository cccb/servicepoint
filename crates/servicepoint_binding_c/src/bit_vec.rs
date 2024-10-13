//! C functions for interacting with [SPBitVec]s
//!
//! prefix `sp_bit_vec_`

use crate::SPByteSlice;
use servicepoint::bitvec::prelude::{BitVec, Msb0};

/// A vector of bits
///
/// # Examples
/// ```C
/// SPBitVec vec = sp_bit_vec_new(8);
/// sp_bit_vec_set(vec, 5, true);
/// sp_bit_vec_free(vec);
/// ```
pub struct SPBitVec(BitVec<u8, Msb0>);

impl From<BitVec<u8, Msb0>> for SPBitVec {
    fn from(actual: BitVec<u8, Msb0>) -> Self {
        Self(actual)
    }
}

impl From<SPBitVec> for BitVec<u8, Msb0> {
    fn from(value: SPBitVec) -> Self {
        value.0
    }
}

impl Clone for SPBitVec {
    fn clone(&self) -> Self {
        SPBitVec(self.0.clone())
    }
}

/// Creates a new [SPBitVec] instance.
///
/// # Arguments
///
/// - `size`: size in bits.
///
/// returns: [SPBitVec] with all bits set to false. Will never return NULL.
///
/// # Panics
///
/// - when `size` is not divisible by 8.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_new(size: usize) -> *mut SPBitVec {
    let result = Box::into_raw(Box::new(SPBitVec(BitVec::repeat(false, size))));
    assert!(!result.is_null());
    result
}

/// Interpret the data as a series of bits and load then into a new [SPBitVec] instance.
///
/// returns: [SPBitVec] instance containing data. Will never return NULL.
///
/// # Panics
///
/// - when `data` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length`
///   bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_load(
    data: *const u8,
    data_length: usize,
) -> *mut SPBitVec {
    assert!(!data.is_null());
    let data = std::slice::from_raw_parts(data, data_length);
    let result = Box::into_raw(Box::new(SPBitVec(BitVec::from_slice(data))));
    assert!(!result.is_null());
    result
}

/// Clones a [SPBitVec].
///
/// returns: new [SPBitVec] instance. Will never return NULL.
///
/// # Panics
///
/// - when `bit_vec` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - `bit_vec` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_clone(
    bit_vec: *const SPBitVec,
) -> *mut SPBitVec {
    assert!(!bit_vec.is_null());
    let result = Box::into_raw(Box::new((*bit_vec).clone()));
    assert!(!result.is_null());
    result
}

/// Deallocates a [SPBitVec].
///
/// # Panics
///
/// - when `but_vec` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - `bit_vec` is not used concurrently or after this call
/// - `bit_vec` was not passed to another consuming function, e.g. to create a [SPCommand]
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_free(bit_vec: *mut SPBitVec) {
    assert!(!bit_vec.is_null());
    _ = Box::from_raw(bit_vec);
}

/// Gets the value of a bit from the [SPBitVec].
///
/// # Arguments
///
/// - `bit_vec`: instance to read from
/// - `index`: the bit index to read
///
/// returns: value of the bit
///
/// # Panics
///
/// - when `bit_vec` is NULL
/// - when accessing `index` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - `bit_vec` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_get(
    bit_vec: *const SPBitVec,
    index: usize,
) -> bool {
    assert!(!bit_vec.is_null());
    *(*bit_vec).0.get(index).unwrap()
}

/// Sets the value of a bit in the [SPBitVec].
///
/// # Arguments
///
/// - `bit_vec`: instance to write to
/// - `index`: the bit index to edit
/// - `value`: the value to set the bit to
///
/// # Panics
///
/// - when `bit_vec` is NULL
/// - when accessing `index` out of bounds
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - `bit_vec` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_set(
    bit_vec: *mut SPBitVec,
    index: usize,
    value: bool,
) {
    assert!(!bit_vec.is_null());
    (*bit_vec).0.set(index, value)
}

/// Sets the value of all bits in the [SPBitVec].
///
/// # Arguments
///
/// - `bit_vec`: instance to write to
/// - `value`: the value to set all bits to
///
/// # Panics
///
/// - when `bit_vec` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - `bit_vec` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_fill(bit_vec: *mut SPBitVec, value: bool) {
    assert!(!bit_vec.is_null());
    (*bit_vec).0.fill(value)
}

/// Gets the length of the [SPBitVec] in bits.
///
/// # Arguments
///
/// - `bit_vec`: instance to write to
///
/// # Panics
///
/// - when `bit_vec` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_len(bit_vec: *const SPBitVec) -> usize {
    assert!(!bit_vec.is_null());
    (*bit_vec).0.len()
}

/// Returns true if length is 0.
///
/// # Arguments
///
/// - `bit_vec`: instance to write to
///
/// # Panics
///
/// - when `bit_vec` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_is_empty(bit_vec: *const SPBitVec) -> bool {
    assert!(!bit_vec.is_null());
    (*bit_vec).0.is_empty()
}

/// Gets an unsafe reference to the data of the [SPBitVec] instance.
///
/// # Arguments
///
/// - `bit_vec`: instance to write to
///
/// # Panics
///
/// - when `bit_vec` is NULL
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `bit_vec` points to a valid [SPBitVec]
/// - the returned memory range is never accessed after the passed [SPBitVec] has been freed
/// - the returned memory range is never accessed concurrently, either via the [SPBitVec] or directly
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_unsafe_data_ref(
    bit_vec: *mut SPBitVec,
) -> SPByteSlice {
    assert!(!bit_vec.is_null());
    let data = (*bit_vec).0.as_raw_mut_slice();
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
