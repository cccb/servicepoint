/// A two-dimensional grid of `T`
pub trait Grid<T> {
    /// Sets the value at the specified position
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell to read
    ///
    /// returns: the old value
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: T) -> T;

    /// Get the current value at the specified position
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get(&self, x: usize, y: usize) -> T;

    /// Get the current value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell to read
    ///
    /// returns: Value at position or None
    fn get_optional(&self, x: isize, y: isize) -> Option<T>;

    /// Sets the value at the specified position if the position is inside of bounds
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell to read
    ///
    /// returns: the old value or None
    fn set_optional(&mut self, x: isize, y: isize, value: T) -> Option<T>;

    /// Sets all cells in the grid to the specified value
    fn fill(&mut self, value: T);

    /// the size in x-direction
    fn width(&self) -> usize;

    /// the height in y-direction
    fn height(&self) -> usize;
}

/// A grid that can return cells as references.
pub trait RefGrid<T> {
    /// Get a reference to the current value at the specified position
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get_ref(&self, x: usize, y: usize) -> &T;

    /// Get a reference to the current value at the specified position if the position is in bounds.
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell
    ///
    /// returns: Reference to cell or None
    fn get_ref_optional(&self, x: isize, y: isize) -> Option<&T>;

    /// Get a mutable reference to the current value at the specified position
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get_ref_mut(&mut self, x: usize, y: usize) -> &mut T;

    /// Get a mutable reference to the current value at the specified position if position is in bounds.
    ///
    /// # Arguments
    ///
    /// * `x` and `y`: position of the cell
    ///
    /// returns: Reference to cell or None
    fn get_ref_mut_optional(&mut self, x: isize, y: isize) -> Option<&mut T>;
}
