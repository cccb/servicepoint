use crate::DataRef;

/// A fixed-size vector of bits
#[derive(Debug, Clone, PartialEq)]
pub struct BitVec {
    size: usize,
    data: Vec<u8>,
}

impl BitVec {
    /// Create a new `BitVec`.
    ///
    /// # Arguments
    ///
    /// * `size`: size in bits.
    ///
    /// returns: `BitVec` with all bits set to false.
    ///
    /// # Panics
    ///
    /// When `size` is not divisible by 8.
    #[must_use]
    pub fn new(size: usize) -> BitVec {
        assert_eq!(size % 8, 0);
        Self {
            size,
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
    ///
    /// # Panics
    ///
    /// When accessing `index` out of bounds.
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
    ///
    /// # Panics
    ///
    /// When accessing `index` out of bounds.
    #[must_use]
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
    ///  use servicepoint::BitVec;
    ///  let mut vec = BitVec::new(8);
    ///  vec.fill(true);
    /// ```
    pub fn fill(&mut self, value: bool) {
        let byte: u8 = if value { 0xFF } else { 0x00 };
        self.data.fill(byte);
    }

    /// Gets the length in bits
    #[must_use]
    pub fn len(&self) -> usize {
        self.size
    }

    /// returns true if length is 0.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get an iterator over every bit in the vector
    pub fn iter(&self) -> Iter {
        Iter {
            bit_vec: self,
            index: 0,
        }
    }

    /// Calculates the byte index and bitmask for a specific bit in the vector
    fn get_indexes(&self, bit_index: usize) -> (usize, u8) {
        assert!(
            bit_index < self.size,
            "bit index {bit_index} is outside of range 0..<{}",
            self.size
        );

        let byte_index = bit_index / 8;
        let bit_in_byte_index = 7 - bit_index % 8;
        let bit_mask: u8 = 1 << bit_in_byte_index;
        (byte_index, bit_mask)
    }
}

impl DataRef for BitVec {
    fn data_ref_mut(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    fn data_ref(&self) -> &[u8] {
        self.data.as_slice()
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
            size: value.len() * 8,
            data: Vec::from(value),
        }
    }
}

pub struct Iter<'t> {
    bit_vec: &'t BitVec,
    index: usize,
}

impl<'t> Iterator for Iter<'t> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bit_vec.size {
            return None;
        }

        let result = Some(self.bit_vec.get(self.index));
        self.index += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{BitVec, DataRef};

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
        assert!(!vec.get(1));
        assert!(!vec.get(11));

        vec.set(1, true);
        vec.set(11, true);
        assert_eq!(vec.data, [0x40, 0x10, 0x00]);
        assert!(!vec.get(0));
        assert!(vec.get(1));
        assert!(vec.get(11));
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
    fn mut_data_ref() {
        let mut vec = BitVec::new(8 * 3);

        let data_ref = vec.data_ref_mut();
        data_ref.copy_from_slice(&[0x40, 0x10, 0x00]);

        assert_eq!(vec.data, [0x40, 0x10, 0x00]);
        assert!(vec.get(1));
    }

    #[test]
    fn is_empty() {
        let vec = BitVec::new(8 * 3);
        assert!(!vec.is_empty());

        let vec = BitVec::new(0);
        assert!(vec.is_empty());
    }

    #[test]
    fn get_returns_old() {
        let mut vec = BitVec::new(8);
        assert!(!vec.set(1, true));
        assert!(vec.set(1, true));
        assert!(vec.set(1, false));
        assert!(!vec.set(1, false));
    }

    #[test]
    fn debug_print() {
        let vec = BitVec::new(8 * 3);
        format!("{vec:?}");
    }
}
