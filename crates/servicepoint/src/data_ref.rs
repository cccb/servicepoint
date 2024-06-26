/// A trait for getting the underlying raw byte slices of data containers.
///
/// The expectation is that you can create an equal instance with this data given the additional
/// metadata needed.
pub trait DataRef<T> {
    /// Get the underlying bytes writable.
    ///
    /// Note that depending on the struct this is implemented on, writing invalid values here might
    /// lead to panics later in the lifetime of the program or on the receiving side.
    fn data_ref_mut(&mut self) -> &mut [T];

    /// Get the underlying bytes read-only.
    fn data_ref(&self) -> &[T];
}
