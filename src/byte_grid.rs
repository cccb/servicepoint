#[derive(Debug)]
pub struct ByteGrid {
    pub width: usize,
    pub height: usize,
    data: Vec<u8>,
}

impl ByteGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data: Vec::from(data),
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[x + y * self.width] = value;
    }

    pub fn fill(&mut self, value: u8){
        self.data.fill(value)
    }
}

impl Into<Vec<u8>> for ByteGrid {
    fn into(self) -> Vec<u8> {
        self.data
    }
}
