/// A vector of bits
#[derive(Clone)]
pub struct BitVec {
    data: Vec<u8>,
}

impl BitVec {
    /// Create a new bit vector.
    ///
    /// # Arguments
    ///
    /// * `size`: size in bits. Must be dividable by 8.
    ///
    /// returns: bit vector with all bits set to false.
    pub fn new(size: usize) -> BitVec {
        assert_eq!(size % 8, 0);
        Self {
            data: vec![0; size / 8],
        }
    }

    /// Sets the value of a bit.
    ///
    /// # Arguments
    ///
    /// * `index`: the bit index to edit
    /// * `value`: the value to set the bit to
    ///
    /// returns: old value of the bit
    pub fn set(&mut self, index: usize, value: bool) -> bool {
        let (byte_index, bit_mask) = self.get_indexes(index);

        let byte = self.data[byte_index];
        let old_value = byte & bit_mask != 0;

        self.data[byte_index] = if value {
            byte | bit_mask
        } else {
            byte & (u8::MAX ^ bit_mask)
        };

        old_value
    }

    /// Gets the value of a bit.
    ///
    /// # Arguments
    ///
    /// * `index`: the bit index to read
    ///
    /// returns: value of the bit
    pub fn get(&self, index: usize) -> bool {
        let (byte_index, bit_mask) = self.get_indexes(index);
        self.data[byte_index] & bit_mask != 0
    }

    /// Sets all bits to the specified value
    ///
    /// # Arguments
    ///
    /// * `value`: the value to set all bits to
    ///
    /// # Examples
    /// ```
    ///  use servicepoint2::BitVec;
    ///  let mut vec = BitVec::new(8);
    ///  vec.fill(true);
    /// ```
    pub fn fill(&mut self, value: bool) {
        let byte: u8 = if value { 0xFF } else { 0x00 };
        self.data.fill(byte);
    }

    /// Gets the length in bits
    pub fn len(&self) -> usize {
        self.data.len() * 8
    }

    /// returns true if length is 0.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get the underlying bits in their raw packed bytes form
    pub fn mut_data_ref(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Calculates the byte index and bitmask for a specific bit in the vector
    fn get_indexes(&self, index: usize) -> (usize, u8) {
        let byte_index = index / 8;
        let bit_in_byte_index = 7 - index % 8;
        let bit_mask: u8 = 1 << bit_in_byte_index;
        (byte_index, bit_mask)
    }
}

impl From<BitVec> for Vec<u8> {
    /// Turns the `BitVec` into the underlying `Vec<u8>`
    fn from(value: BitVec) -> Self {
        value.data
    }
}

impl From<&[u8]> for BitVec {
    /// Interpret the data as a series of bits and load then into a new `BitVec` instance.
    fn from(value: &[u8]) -> Self {
        Self {
            data: Vec::from(value),
        }
    }
}

impl std::fmt::Debug for BitVec {
    /// Formats a `BitVec` for debug. The manual implementation includes the length of the instance.
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BitVec")
            .field("len", &self.len())
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    use crate::{BitVec, CByteSlice};

    /// Creates a new `BitVec` instance.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_new(size: usize) -> *mut BitVec {
        Box::into_raw(Box::new(BitVec::new(size)))
    }

    /// Loads a `BitVec` from the provided data.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_load(
        data: *const u8,
        data_length: usize,
    ) -> *mut BitVec {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(BitVec::from(data)))
    }

    /// Clones a `BitVec`.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_clone(
        this: *const BitVec,
    ) -> *mut BitVec {
        Box::into_raw(Box::new((*this).clone()))
    }

    /// Deallocates a `BitVec`.
    ///
    /// Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_dealloc(this: *mut BitVec) {
        _ = Box::from_raw(this);
    }

    /// Gets the value of a bit from the `BitVec`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_get(
        this: *const BitVec,
        index: usize,
    ) -> bool {
        (*this).get(index)
    }

    /// Sets the value of a bit in the `BitVec`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_set(
        this: *mut BitVec,
        index: usize,
        value: bool,
    ) -> bool {
        (*this).set(index, value)
    }

    /// Sets the value of all bits in the `BitVec`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_fill(this: *mut BitVec, value: bool) {
        (*this).fill(value)
    }

    /// Gets the length of the `BitVec` in bits.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_len(this: *const BitVec) -> usize {
        (*this).len()
    }

    /// Returns true if length is 0.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_is_empty(this: *const BitVec) -> bool {
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
    pub unsafe extern "C" fn sp2_bit_vec_unsafe_data_ref(
        this: *mut BitVec,
    ) -> CByteSlice {
        let data = (*this).mut_data_ref();
        CByteSlice {
            start: data.as_mut_ptr_range().start,
            length: data.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BitVec;

    #[test]
    fn fill() {
        let mut vec = BitVec::new(8 * 3);
        assert_eq!(vec.data, [0x00, 0x00, 0x00]);

        vec.fill(true);
        assert_eq!(vec.data, [0xFF, 0xFF, 0xFF]);

        vec.fill(false);
        assert_eq!(vec.data, [0x00, 0x00, 0x00]);
    }

    #[test]
    fn get_set() {
        let mut vec = BitVec::new(8 * 3);
        assert_eq!(vec.get(1), false);
        assert_eq!(vec.get(11), false);

        vec.set(1, true);
        vec.set(11, true);
        assert_eq!(vec.data, [0x40, 0x10, 0x00]);
        assert_eq!(vec.get(0), false);
        assert_eq!(vec.get(1), true);
        assert_eq!(vec.get(11), true);
    }

    #[test]
    fn load() {
        let mut vec = BitVec::new(8 * 3);
        vec.set(6, true);
        vec.set(7, true);
        vec.set(8, true);
        vec.set(9, true);
        vec.set(10, true);
        vec.set(vec.len() - 1, true);

        assert_eq!(vec.data, [0x03, 0xE0, 0x01]);

        let data: Vec<u8> = vec.into();

        let vec = BitVec::from(&*data);
        assert_eq!(vec.data, [0x03, 0xE0, 0x01]);
    }

    #[test]
    fn mut_data_ref(){
        let mut vec = BitVec::new(8 * 3);

        let data_ref = vec.mut_data_ref();
        data_ref.copy_from_slice(&[0x40, 0x10, 0x00]);

        assert_eq!(vec.data, [0x40, 0x10, 0x00]);
        assert_eq!(vec.get(1), true);
    }

    #[test]
    fn is_empty() {
        let vec = BitVec::new(8 * 3);
        assert_eq!(vec.is_empty(), false);

        let vec = BitVec::new(0);
        assert_eq!(vec.is_empty(), true);
    }
}