//! C functions for interacting with `BitVec`s
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
/// sp_bit_vec_dealloc(vec);
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

/// Creates a new `BitVec` instance.
///
/// # Arguments
///
/// - `size`: size in bits.
///
/// returns: `BitVec` with all bits set to false.
///
/// # Panics
///
/// When `size` is not divisible by 8.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_new(size: usize) -> *mut SPBitVec {
    Box::into_raw(Box::new(SPBitVec(BitVec::repeat(false, size))))
}

/// Interpret the data as a series of bits and load then into a new `BitVec` instance.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory location of at least `data_length`
///   bytes in size.
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_load(
    data: *const u8,
    data_length: usize,
) -> *mut SPBitVec {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(SPBitVec(BitVec::from_slice(data))))
}

/// Clones a `BitVec`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_clone(
    this: *const SPBitVec,
) -> *mut SPBitVec {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `BitVec`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not used concurrently or after this call
/// - `this` was not passed to another consuming function, e.g. to create a `Command`
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_dealloc(this: *mut SPBitVec) {
    _ = Box::from_raw(this);
}

/// Gets the value of a bit from the `BitVec`.
///
/// # Arguments
///
/// - `this`: instance to read from
/// - `index`: the bit index to read
///
/// returns: value of the bit
///
/// # Panics
///
/// When accessing `index` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not written to concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_get(
    this: *const SPBitVec,
    index: usize,
) -> bool {
    *(*this).0.get(index).unwrap()
}

/// Sets the value of a bit in the `BitVec`.
///
/// # Arguments
///
/// - `this`: instance to write to
/// - `index`: the bit index to edit
/// - `value`: the value to set the bit to
///
/// returns: old value of the bit
///
/// # Panics
///
/// When accessing `index` out of bounds.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_set(
    this: *mut SPBitVec,
    index: usize,
    value: bool,
) {
    (*this).0.set(index, value)
}

/// Sets the value of all bits in the `BitVec`.
///
/// # Arguments
///
/// - `value`: the value to set all bits to
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - `this` is not written to or read from concurrently
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_fill(this: *mut SPBitVec, value: bool) {
    (*this).0.fill(value)
}

/// Gets the length of the `BitVec` in bits.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_len(this: *const SPBitVec) -> usize {
    (*this).0.len()
}

/// Returns true if length is 0.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_is_empty(this: *const SPBitVec) -> bool {
    (*this).0.is_empty()
}

/// Gets an unsafe reference to the data of the `BitVec` instance.
///
/// ## Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `BitVec`
/// - the returned memory range is never accessed after the passed `BitVec` has been freed
/// - the returned memory range is never accessed concurrently, either via the `BitVec` or directly
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_unsafe_data_ref(
    this: *mut SPBitVec,
) -> SPByteSlice {
    let data = (*this).0.as_raw_mut_slice();
    SPByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
