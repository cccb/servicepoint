use crate::{BitVec, DataRef, Grid, PIXEL_HEIGHT, PIXEL_WIDTH};

/// A grid of pixels stored in packed bytes.
#[derive(Debug, Clone, PartialEq)]
pub struct PixelGrid {
    width: usize,
    height: usize,
    bit_vec: BitVec,
}

impl PixelGrid {
    /// Creates a new pixel grid with the size of the whole screen.
    #[must_use]
    pub fn max_sized() -> Self {
        Self::new(PIXEL_WIDTH, PIXEL_HEIGHT)
    }

    /// Loads a `PixelGrid` with the specified dimensions from the provided data.
    ///
    /// # Arguments
    ///
    /// * `width`: size in pixels in x-direction
    /// * `height`: size in pixels in y-direction
    ///
    /// returns: `PixelGrid` that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    /// - when the width is not dividable by 8
    #[must_use]
    pub fn load(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(width % 8, 0);
        assert_eq!(data.len(), height * width / 8);
        Self {
            width,
            height,
            bit_vec: BitVec::from(data),
        }
    }

    fn check_indexes(&self, x: usize, y: usize) {
        assert!(
            x < self.width,
            "cannot access pixel {x}-{y} because x is outside of bounds 0..{}",
            self.width
        );
        assert!(
            y < self.height,
            "cannot access pixel {x}-{y} because y is outside of bounds 0..{}",
            self.height
        );
    }
}

impl Grid<bool> for PixelGrid {
    /// Creates a new `PixelGrid` with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width`: size in pixels in x-direction
    /// * `height`: size in pixels in y-direction
    ///
    /// returns: `PixelGrid` initialized to all pixels off
    ///
    /// # Panics
    ///
    /// - when the width is not dividable by 8
    fn new(width: usize, height: usize) -> Self {
        assert_eq!(width % 8, 0);
        Self {
            width,
            height,
            bit_vec: BitVec::new(width * height),
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        self.check_indexes(x, y);
        self.bit_vec.set(x + y * self.width, value)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.bit_vec.get(x + y * self.width)
    }

    fn fill(&mut self, value: bool) {
        self.bit_vec.fill(value);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn window(&self, x: usize, y: usize, w: usize, h: usize) -> Self {
        // TODO: how to deduplicate?
        // this cannot be moved into the trait because there, Self is not Sized
        let mut win = Self::new(w, h);
        for win_x in 0..w {
            for win_y in 0..h {
                let value = self.get(x + win_x, y + win_y);
                win.set(win_x, win_y, value);
            }
        }
        win
    }
}

impl DataRef for PixelGrid {
    fn data_ref_mut(&mut self) -> &mut [u8] {
        self.bit_vec.data_ref_mut()
    }

    fn data_ref(&self) -> &[u8] {
        self.bit_vec.data_ref()
    }
}

impl From<PixelGrid> for Vec<u8> {
    /// Turns a `PixelGrid` into the underlying `Vec<u8>`.
    fn from(value: PixelGrid) -> Self {
        value.bit_vec.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{DataRef, Grid, PixelGrid};

    #[test]
    fn fill() {
        let mut grid = PixelGrid::new(8, 2);
        assert_eq!(grid.data_ref(), [0x00, 0x00]);

        grid.fill(true);
        assert_eq!(grid.data_ref(), [0xFF, 0xFF]);

        grid.fill(false);
        assert_eq!(grid.data_ref(), [0x00, 0x00]);
    }

    #[test]
    fn get_set() {
        let mut grid = PixelGrid::new(8, 2);
        assert!(!grid.get(0, 0));
        assert!(!grid.get(1, 1));

        grid.set(5, 0, true);
        grid.set(1, 1, true);
        assert_eq!(grid.data_ref(), [0x04, 0x40]);

        assert!(grid.get(5, 0));
        assert!(grid.get(1, 1));
        assert!(!grid.get(1, 0));
    }

    #[test]
    fn load() {
        let mut grid = PixelGrid::new(8, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) % 2 == 0);
            }
        }

        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);

        let data: Vec<u8> = grid.into();

        let grid = PixelGrid::load(8, 3, &data);
        assert_eq!(grid.data_ref(), [0xAA, 0x55, 0xAA]);
    }
}
