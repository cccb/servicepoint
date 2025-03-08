/// A two-dimensional grid of `T`
pub trait Grid<T> {
    /// Sets the value at the specified position
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: T);

    /// Get the current value at the specified position
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get(&self, x: usize, y: usize) -> T;

    /// Get the current value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: Value at position or None
    fn get_optional(&self, x: isize, y: isize) -> Option<T> {
        if self.is_in_bounds(x, y) {
            Some(self.get(x as usize, y as usize))
        } else {
            None
        }
    }

    /// Sets the value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: the old value or None
    fn set_optional(&mut self, x: isize, y: isize, value: T) -> bool {
        if self.is_in_bounds(x, y) {
            self.set(x as usize, y as usize, value);
            true
        } else {
            false
        }
    }

    /// Sets all cells in the grid to the specified value
    fn fill(&mut self, value: T);

    /// the size in x-direction
    fn width(&self) -> usize;

    /// the height in y-direction
    fn height(&self) -> usize;

    /// Checks whether the specified signed position is in grid bounds
    fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0
            && x < self.width() as isize
            && y >= 0
            && y < self.height() as isize
    }

    /// Asserts that the specified unsigned position is in grid bounds.
    ///
    /// # Panics
    ///
    /// When the specified position is out of bounds for this grid.
    fn assert_in_bounds(&self, x: usize, y: usize) {
        let width = self.width();
        assert!(x < width, "cannot access index [{x}, {y}] because x is outside of bounds [0..{width})");
        let height = self.height();
        assert!(y < height, "cannot access index [{x}, {y}] because y is outside of bounds [0..{height})");
    }
}
