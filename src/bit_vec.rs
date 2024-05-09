/// A vector of bits
#[derive(Debug)]
pub struct BitVec {
    data: Vec<u8>,
}

impl BitVec {
    pub fn new(size: usize) -> BitVec {
        assert_eq!(size % 8, 0);
        Self { data: vec!(0; size / 8) }
    }

    pub fn set(&mut self, index: usize, value: bool) -> bool {
        let byte_index = index / 8;
        let bit_in_byte_index = 7 - index % 8;
        let bit_mask = 1 << bit_in_byte_index;

        let byte = self.data[byte_index];
        let old_value = byte & bit_mask != 0;

        self.data[byte_index] = if value {
            byte | bit_mask
        } else {
            byte ^ bit_mask
        };

        return old_value;
    }

    pub fn get(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_in_byte_index = 7 - index % 8;
        let bit_mask = 1 << bit_in_byte_index;
        return self.data[byte_index] & bit_mask != 0;
    }
}

impl Into<Vec<u8>> for BitVec {
    fn into(self) -> Vec<u8> {
        self.data
    }
}
