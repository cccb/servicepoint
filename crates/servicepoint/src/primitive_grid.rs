use std::slice::{Iter, IterMut};

use crate::{DataRef, Grid};

pub trait PrimitiveGridType: Sized + Default + Copy + Clone {}
impl<T: Sized + Default + Copy + Clone> PrimitiveGridType for T {}

/// A 2D grid of bytes
#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveGrid<T: PrimitiveGridType> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: PrimitiveGridType> PrimitiveGrid<T> {
    /// Creates a new [PrimitiveGrid] with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// - width: size in x-direction
    /// - height: size in y-direction
    ///
    /// returns: [PrimitiveGrid] initialized to default value.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![Default::default(); width * height],
            width,
            height,
        }
    }

    /// Loads a [PrimitiveGrid] with the specified dimensions from the provided data.
    ///
    /// returns: [PrimitiveGrid] that contains a copy of the provided data
    ///
    /// # Panics
    ///
    /// - when the dimensions and data size do not match exactly.
    #[must_use]
    pub fn load(width: usize, height: usize, data: &[T]) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            data: Vec::from(data),
            width,
            height,
        }
    }

    /// Iterate over all cells in [PrimitiveGrid].
    ///
    /// Order is equivalent to the following loop:
    /// ```
    /// # use servicepoint::{PrimitiveGrid, Grid};
    /// # let grid = PrimitiveGrid::<u8>::new(2,2);
    /// for y in 0..grid.height() {
    ///     for x in 0..grid.width() {
    ///         grid.get(x, y);
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    /// Iterate over all rows in [PrimitiveGrid] top to bottom.
    pub fn iter_rows(&self) -> IterRows<T> {
        IterRows {
            byte_grid: self,
            row: 0,
        }
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all cells from top left to bottom right.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    /// Get a mutable reference to the current value at the specified position.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    pub fn get_ref_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.assert_in_bounds(x, y);
        &mut self.data[x + y * self.width]
    }

    /// Get a mutable reference to the current value at the specified position if position is in bounds.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    ///
    /// returns: Reference to cell or None
    pub fn get_ref_mut_optional(
        &mut self,
        x: isize,
        y: isize,
    ) -> Option<&mut T> {
        if self.is_in_bounds(x, y) {
            Some(&mut self.data[x as usize + y as usize * self.width])
        } else {
            None
        }
    }
}

impl<T: PrimitiveGridType> Grid<T> for PrimitiveGrid<T> {
    /// Sets the value of the cell at the specified position in the `PrimitiveGrid.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell
    /// - `value`: the value to write to the cell
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.assert_in_bounds(x, y);
        self.data[x + y * self.width] = value;
    }

    /// Gets the current value at the specified position.
    ///
    /// # Arguments
    ///
    /// - `x` and `y`: position of the cell to read
    ///
    /// # Panics
    ///
    /// When accessing `x` or `y` out of bounds.
    fn get(&self, x: usize, y: usize) -> T {
        self.assert_in_bounds(x, y);
        self.data[x + y * self.width]
    }

    fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T: PrimitiveGridType> DataRef<T> for PrimitiveGrid<T> {
    /// Get the underlying byte rows mutable
    fn data_ref_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Get the underlying byte rows read only
    fn data_ref(&self) -> &[T] {
        self.data.as_slice()
    }
}

impl<T: PrimitiveGridType> From<PrimitiveGrid<T>> for Vec<T> {
    /// Turn into the underlying [`Vec<u8>`] containing the rows of bytes.
    fn from(value: PrimitiveGrid<T>) -> Self {
        value.data
    }
}

pub struct IterRows<'t, T: PrimitiveGridType> {
    byte_grid: &'t PrimitiveGrid<T>,
    row: usize,
}

impl<'t, T: PrimitiveGridType> Iterator for IterRows<'t, T> {
    type Item = Iter<'t, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.byte_grid.height {
            return None;
        }

        let start = self.row * self.byte_grid.width;
        let end = start + self.byte_grid.width;
        let result = self.byte_grid.data[start..end].iter();
        self.row += 1;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{DataRef, Grid, PrimitiveGrid};

    #[test]
    fn fill() {
        let mut grid = PrimitiveGrid::<usize>::new(2, 2);
        assert_eq!(grid.data, [0x00, 0x00, 0x00, 0x00]);

        grid.fill(42);
        assert_eq!(grid.data, [42; 4]);
    }

    #[test]
    fn get_set() {
        let mut grid = PrimitiveGrid::new(2, 2);
        assert_eq!(grid.get(0, 0), 0);
        assert_eq!(grid.get(1, 1), 0);

        grid.set(0, 0, 42);
        grid.set(1, 0, 23);
        assert_eq!(grid.data, [42, 23, 0, 0]);

        assert_eq!(grid.get(0, 0), 42);
        assert_eq!(grid.get(1, 0), 23);
        assert_eq!(grid.get(1, 1), 0);
    }

    #[test]
    fn load() {
        let mut grid = PrimitiveGrid::new(2, 3);
        for x in 0..grid.width {
            for y in 0..grid.height {
                grid.set(x, y, (x + y) as u8);
            }
        }

        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);

        let data: Vec<u8> = grid.into();

        let grid = PrimitiveGrid::load(2, 3, &data);
        assert_eq!(grid.data, [0, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn mut_data_ref() {
        let mut vec = PrimitiveGrid::new(2, 2);

        let data_ref = vec.data_ref_mut();
        data_ref.copy_from_slice(&[1, 2, 3, 4]);

        assert_eq!(vec.data, [1, 2, 3, 4]);
        assert_eq!(vec.get(1, 0), 2)
    }

    #[test]
    fn iter() {
        let mut vec = PrimitiveGrid::new(2, 2);
        vec.set(1, 1, 5);

        let mut iter = vec.iter();
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 0);
        assert_eq!(*iter.next().unwrap(), 5);
    }

    #[test]
    fn iter_mut() {
        let mut vec = PrimitiveGrid::new(2, 3);
        for (index, cell) in vec.iter_mut().enumerate() {
            *cell = index as u8;
        }

        assert_eq!(vec.data_ref(), [0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter_rows() {
        let vec = PrimitiveGrid::load(2, 3, &[0, 1, 1, 2, 2, 3]);
        for (y, row) in vec.iter_rows().enumerate() {
            for (x, val) in row.enumerate() {
                assert_eq!(*val, (x + y) as u8);
            }
        }
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_x() {
        let mut vec = PrimitiveGrid::load(2, 2, &[0, 1, 2, 3]);
        vec.set(2, 1, 5);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_y() {
        let vec = PrimitiveGrid::load(2, 2, &[0, 1, 2, 3]);
        vec.get(1, 2);
    }

    #[test]
    fn ref_mut() {
        let mut vec = PrimitiveGrid::load(2, 2, &[0, 1, 2, 3]);

        let top_left = vec.get_ref_mut(0, 0);
        *top_left += 5;

        assert_eq!(None, vec.get_ref_mut_optional(2, 2));
        assert_eq!(Some(&mut 5), vec.get_ref_mut_optional(0, 0));
    }

    #[test]
    fn optional() {
        let mut grid = PrimitiveGrid::load(2, 2, &[0, 1, 2, 3]);
        grid.set_optional(0, 0, 5);
        grid.set_optional(-1, 0, 8);
        grid.set_optional(0, 8, 42);
        assert_eq!(grid.data, [5, 1, 2, 3]);

        assert_eq!(grid.get_optional(0, 0), Some(5));
        assert_eq!(grid.get_optional(0, 8), None);
    }
}
