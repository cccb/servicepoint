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

        return old_value;
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
        return self.data[byte_index] & bit_mask != 0;
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

    pub fn len(&self) -> usize {
        self.data.len() * 8
    }

    pub fn data_ref(&self) -> &[u8] {
        &*self.data
    }

    fn get_indexes(&self, index: usize) -> (usize, u8) {
        let byte_index = index / 8;
        let bit_in_byte_index = 7 - index % 8;
        let bit_mask: u8 = 1 << bit_in_byte_index;
        return (byte_index, bit_mask);
    }
}

impl Into<Vec<u8>> for BitVec {
    fn into(self) -> Vec<u8> {
        self.data
    }
}

impl From<&[u8]> for BitVec {
    fn from(value: &[u8]) -> Self {
        Self {
            data: Vec::from(value),
        }
    }
}

impl std::fmt::Debug for BitVec {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("BitVec")
            .field("len", &self.len())
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(feature = "c-api")]
pub mod c_api {
    use crate::BitVec;

    /// Creates a new `BitVec` instance.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_new(size: usize) -> *mut BitVec {
        Box::into_raw(Box::new(BitVec::new(size)))
    }

    /// Loads a `BitVec` from the provided data.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_load(data: *const u8, data_length: usize) -> *mut BitVec {
        let data = std::slice::from_raw_parts(data, data_length);
        Box::into_raw(Box::new(BitVec::from(data)))
    }

    /// Clones a `BitVec`.
    /// The returned instance has to be freed with `bit_vec_dealloc`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_clone(this: *const BitVec) -> *mut BitVec {
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
    pub unsafe extern "C" fn sp2_bit_vec_get(this: *const BitVec, index: usize) -> bool {
        (*this).get(index)
    }

    /// Sets the value of a bit in the `BitVec`.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_bit_vec_set(this: *mut BitVec, index: usize, value: bool) -> bool {
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
}
