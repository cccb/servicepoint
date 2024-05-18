pub trait Grid<T> {
    fn new(width: usize, height: usize) -> Self;

    /// Sets the value at the specified position
    ///
    /// returns: the old value
    fn set(&mut self, x: usize, y: usize, value: T) -> T;

    /// Get the current value at the specified position
    fn get(&self, x: usize, y: usize) -> T;

    /// Sets all cells in the grid to the specified value
    fn fill(&mut self, value: T);

    /// the size in x-direction
    fn width(&self) -> usize;

    /// the height in y-direction
    fn height(&self) -> usize;

    /// Creates a new instance containing the specified window.
    ///
    /// Use concrete types to avoid boxing.
    ///
    /// # Arguments
    ///
    /// * `x`: column of the top left cell
    /// * `y`: row of the top left cell
    /// * `w`: size of window in x-direction
    /// * `h`: size of window in y-direction
    ///
    /// returns: Self
    ///
    /// # Examples
    /// To avoid boxing, this example is using the concrete type `ByteGrid`.
    /// ```
    /// use servicepoint2::{ByteGrid, Grid};
    /// fn split(grid: ByteGrid) -> (ByteGrid, ByteGrid) {
    ///     assert!(grid.width() >= 2);
    ///     let split_x = grid.width() / 2;
    ///     let right_w = grid.width() - split_x;
    ///
    ///     let left = grid.window(0, 0, split_x, grid.height());
    ///     let right = grid.window(split_x, 0, right_w, grid.height());
    ///     (left, right)
    /// }
    ///
    /// let (l, r) = split(ByteGrid::new(9, 5));
    /// ```
    fn window(&self, x: usize, y: usize, w: usize, h: usize) -> Self;
}
