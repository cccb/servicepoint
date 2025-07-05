use crate::Grid;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct WindowMut<'t, TElement: Copy, TGrid: Grid<TElement>> {
    grid: &'t mut TGrid,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    phantom: PhantomData<TElement>,
}

impl<'t, TElement: Copy, TGrid: Grid<TElement>> WindowMut<'t, TElement, TGrid> {
    #[must_use]
    pub fn new(
        grid: &'t mut TGrid,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<Self> {
        if width == 0 || height == 0 {
            return None;
        }
        if !grid.is_in_bounds(x + width - 1, y + height - 1) {
            return None;
        }

        Some(Self {
            grid,
            x,
            y,
            width,
            height,
            phantom: PhantomData,
        })
    }
}

impl<TElement: Copy, TGrid: Grid<TElement>> Grid<TElement>
    for WindowMut<'_, TElement, TGrid>
{
    fn set(&mut self, x: usize, y: usize, value: TElement) {
        self.grid.set(x + self.x, y + self.y, value)
    }
    fn get(&self, x: usize, y: usize) -> TElement {
        self.grid.get(x + self.x, y + self.y)
    }

    fn fill(&mut self, value: TElement) {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                self.grid.set(x, y, value);
            }
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use crate::containers::grid_view::WindowMut;
    use crate::{Bitmap, CharGrid, DataRef, Grid};

    #[test]
    fn test_grid_view_bitmap() {
        let mut bitmap = Bitmap::new(8, 4).unwrap();

        // non-byte-aligned views work
        let mut view = WindowMut::new(&mut bitmap, 3, 1, 4, 2).unwrap();
        view.fill(true);

        assert_eq!(bitmap.data_ref(), &[0, 30, 30, 0]);

        // full size view works
        _ = WindowMut::new(&mut bitmap, 0, 0, 8, 4).unwrap();

        // zero size view does not work
        assert!(WindowMut::new(&mut bitmap, 1, 2, 3, 0).is_none());
        assert!(WindowMut::new(&mut bitmap, 1, 2, 0, 1).is_none());

        // oob does not work
        assert!(WindowMut::new(&mut bitmap, 30, 43, 3, 1).is_none());
        assert!(WindowMut::new(&mut bitmap, 0, 0, 9, 1).is_none());
        assert!(WindowMut::new(&mut bitmap, 0, 0, 1, 5).is_none());
    }

    #[test]
    fn test_grid_view_char_grid() {
        let mut grid = CharGrid::new(3, 4);
        grid.fill(' ');

        let mut view = WindowMut::new(&mut grid, 1, 1, 1, 2).unwrap();
        view.fill('#');

        assert_eq!(
            grid.data_ref(),
            &[' ', ' ', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', ' ', ' ']
        );

        // full size view works
        _ = WindowMut::new(&mut grid, 0, 0, 3, 4).unwrap();

        // zero size view does not work
        assert!(WindowMut::new(&mut grid, 1, 2, 2, 0).is_none());
        assert!(WindowMut::new(&mut grid, 1, 2, 0, 1).is_none());
    }

    #[test]
    fn round_trip_bitmap() {
        let mut bitmap = Bitmap::new(8, 4).unwrap();

        let non_aligned = WindowMut::new(&mut bitmap, 3, 1, 4, 2).unwrap();
        assert_eq!(Bitmap::try_from(&non_aligned), Err(()));

        let aligned = WindowMut::new(&mut bitmap, 0, 1, 8, 2).unwrap();

        assert!(matches!(Bitmap::try_from(&aligned), Ok(_)));
    }
}
