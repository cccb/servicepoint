pub use servicepoint::BitVec;
use servicepoint::DataRef;

use crate::c_slice::CByteSlice;

/// Creates a new `BitVec` instance.
/// The returned instance has to be freed with `bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_new(size: usize) -> *mut BitVec {
    Box::into_raw(Box::new(BitVec::new(size)))
}

/// Loads a `BitVec` from the provided data.
/// The returned instance has to be freed with `bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_load(
    data: *const u8,
    data_length: usize,
) -> *mut BitVec {
    let data = std::slice::from_raw_parts(data, data_length);
    Box::into_raw(Box::new(BitVec::from(data)))
}

/// Clones a `BitVec`.
/// The returned instance has to be freed with `bit_vec_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_clone(this: *const BitVec) -> *mut BitVec {
    Box::into_raw(Box::new((*this).clone()))
}

/// Deallocates a `BitVec`.
///
/// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_dealloc(this: *mut BitVec) {
    _ = Box::from_raw(this);
}

/// Gets the value of a bit from the `BitVec`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_get(
    this: *const BitVec,
    index: usize,
) -> bool {
    (*this).get(index)
}

/// Sets the value of a bit in the `BitVec`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_set(
    this: *mut BitVec,
    index: usize,
    value: bool,
) -> bool {
    (*this).set(index, value)
}

/// Sets the value of all bits in the `BitVec`.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_fill(this: *mut BitVec, value: bool) {
    (*this).fill(value)
}

/// Gets the length of the `BitVec` in bits.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_len(this: *const BitVec) -> usize {
    (*this).len()
}

/// Returns true if length is 0.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_is_empty(this: *const BitVec) -> bool {
    (*this).is_empty()
}

/// Gets an unsafe reference to the data of the `BitVec` instance.
///
/// ## Safety
///
/// The caller has to make sure to never access the returned memory after the `BitVec`
/// instance has been consumed or manually deallocated.
///
/// Reading and writing concurrently to either the original instance or the returned data will
/// result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn sp_bit_vec_unsafe_data_ref(
    this: *mut BitVec,
) -> CByteSlice {
    let data = (*this).data_ref_mut();
    CByteSlice {
        start: data.as_mut_ptr_range().start,
        length: data.len(),
    }
}
