//! FFI slice helper

use std::ptr::NonNull;

#[repr(C)]
/// Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
///
/// You should not create an instance of this type in your C code.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - accesses to the memory pointed to with `start` is never accessed outside `length`
/// - the lifetime of the `CByteSlice` does not outlive the memory it points to, as described in
///   the function returning this type.
/// - an instance of this created from C is never passed to a consuming function, as the rust code
///   will try to free the memory of a potentially separate allocator.
pub struct SPByteSlice {
    /// The start address of the memory
    pub start: NonNull<u8>,
    /// The amount of memory in bytes
    pub length: usize,
}
