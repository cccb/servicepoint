//! FFI slice helper

#[repr(C)]
/// Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - accesses to the memory pointed to with `start` is never accessed outside `length`
/// - the lifetime of the `CByteSlice` does not outlive the memory it points to, as described in
///   the function returning this type.
pub struct SPByteSlice {
    /// The start address of the memory
    pub start: *mut u8,
    /// The amount of memory in bytes
    pub length: usize,
}
