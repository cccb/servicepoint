pub trait Grid<T> {
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
}
