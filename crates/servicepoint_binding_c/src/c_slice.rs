#[repr(C)]
/// Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
///
/// Usage of this type is inherently unsafe.
pub struct CByteSlice {
    /// The start address of the memory
    pub start: *mut u8,
    /// The amount of memory in bytes
    pub length: usize,
}
