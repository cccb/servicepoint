/// A two-dimensional readonly grid of `T`
pub trait Grid<T> {
    /// Get the current value at the specified position
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    #[must_use]
    fn get(&self, x: usize, y: usize) -> T;

    /// Get the current value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: Value at position or None
    #[must_use]
    fn get_optional(&self, x: usize, y: usize) -> Option<T> {
        if self.is_in_bounds(x, y) {
            Some(self.get(x, y))
        } else {
            None
        }
    }

    /// the size in x-direction
    #[must_use]
    fn width(&self) -> usize;

    /// the height in y-direction
    #[must_use]
    fn height(&self) -> usize;

    /// Checks whether the specified signed position is in grid bounds
    #[must_use]
    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
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

    /// Copies a row from the grid.
    ///
    /// Returns [None] if y is out of bounds.
    #[must_use]
    fn get_row(&self, y: usize) -> Option<Vec<T>> {
        if y >= self.height() {
            return None;
        }

        let width = self.width();
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            row.push(self.get(x, y));
        }
        Some(row)
    }

    /// Copies a column from the grid.
    ///
    /// Returns [None] if x is out of bounds.
    #[must_use]
    fn get_col(&self, x: usize) -> Option<Vec<T>> {
        if x >= self.width() {
            return None;
        }

        let height = self.height();
        let mut col = Vec::with_capacity(height);
        for y in 0..height {
            col.push(self.get(x, y));
        }
        Some(col)
    }
}

/// A two-dimensional mutable grid of `T`
pub trait GridMut<T>: Grid<T> {
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

    /// Sets the value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// returns: true if the value has been set
    #[must_use]
    fn set_optional(&mut self, x: usize, y: usize, value: T) -> bool {
        if self.is_in_bounds(x, y) {
            self.set(x, y, value);
            true
        } else {
            false
        }
    }

    /// Sets all cells in the grid to the specified value
    fn fill(&mut self, value: T);

    /// Fills the grid with the values from the provided grid.
    ///
    /// The grids have to match in size exactly.
    ///
    /// For 1D slices the equivalent would be `*slice = other_slice`.
    fn deref_assign<O: Grid<T>>(&mut self, other: &O) {
        let width = self.width();
        let height = self.height();
        assert_eq!(
            width,
            other.width(),
            "Cannot assign grid of width {} to a window of width {}",
            other.width(),
            self.width()
        );
        assert_eq!(
            height,
            other.height(),
            "Cannot assign grid of height {} to a height of width {}",
            other.height(),
            self.height()
        );
        for y in 0..height {
            for x in 0..width {
                self.set(x, y, other.get(x, y));
            }
        }
    }
}
