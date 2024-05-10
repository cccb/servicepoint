use crate::{BitVec, PIXEL_HEIGHT, PIXEL_WIDTH};

#[derive(Debug, Clone)]
pub struct PixelGrid {
    pub width: usize,
    pub height: usize,
    bit_vec: BitVec,
}

impl PixelGrid {
    pub fn new(width: usize, height: usize) -> Self {
        assert_eq!(width % 8, 0);
        assert_eq!(height % 8, 0);
        Self {
            width,
            height,
            bit_vec: BitVec::new(width * height),
        }
    }

    pub fn max_sized() -> Self {
        Self::new(PIXEL_WIDTH as usize, PIXEL_HEIGHT as usize)
    }

    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width % 8, 0);
        assert_eq!(height % 8, 0);
        assert_eq!(data.len(), height * width / 8);
        Self {
            width,
            height,
            bit_vec: BitVec::load(data),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        self.bit_vec.set(x + y * self.width, value)
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bit_vec.get(x + y * self.width)
    }

    pub fn fill(&mut self, value: bool) {
        self.bit_vec.fill(value);
    }
}

impl Into<Vec<u8>> for PixelGrid {
    fn into(self) -> Vec<u8> {
        self.bit_vec.into()
    }
}
