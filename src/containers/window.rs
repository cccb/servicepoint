use crate::{Grid, GridMut};
use std::marker::PhantomData;

macro_rules! define_window {
    ($name:ident, $grid:ty) => {
        /// A window into a 2D grid.
        ///
        /// All operations are done directly on the contained reference,
        /// but translated to where the window is.
        #[derive(Debug)]
        pub struct $name<'t, TElement: Copy, TGrid: Grid<TElement>> {
            grid: $grid,
            x: usize,
            y: usize,
            width: usize,
            height: usize,
            phantom: PhantomData<TElement>,
        }

        impl<'t, TElement: Copy, TGrid: Grid<TElement>>
            $name<'t, TElement, TGrid>
        {
            /// Create a new window into `grid`.
            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn new(
                grid: $grid,
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

        #[inherent::inherent]
        impl<TElement: Copy, TGrid: Grid<TElement>> Grid<TElement>
            for $name<'_, TElement, TGrid>
        {
            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn get(&self, x: usize, y: usize) -> TElement {
                self.grid.get(x + self.x, y + self.y)
            }

            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn width(&self) -> usize {
                self.width
            }

            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn height(&self) -> usize {
                self.height
            }
        }
    };
}

define_window!(Window, &'t TGrid);
define_window!(WindowMut, &'t mut TGrid);

#[inherent::inherent]
impl<TElement: Copy, TGrid: GridMut<TElement>> GridMut<TElement>
    for WindowMut<'_, TElement, TGrid>
{
    #[allow(unused, reason = "False positive because of #[inherent]")]
    pub fn set(&mut self, x: usize, y: usize, value: TElement) {
        self.grid.set(x + self.x, y + self.y, value);
    }

    #[allow(unused, reason = "False positive because of #[inherent]")]
    pub fn fill(&mut self, value: TElement) {
        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                self.grid.set(x, y, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::containers::window::{Window, WindowMut};
    use crate::{Bitmap, CharGrid, DataRef, GridMut};

    #[test]
    fn test_grid_view_bitmap() {
        let mut bitmap = Bitmap::new(8, 4).unwrap();

        // non-byte-aligned views work
        let mut view = bitmap.window_mut(3, 1, 4, 2).unwrap();
        view.fill(true);

        assert_eq!(bitmap.data_ref(), &[0, 30, 30, 0]);

        assert_eq!(bitmap.set_optional(99, 99, false), false);

        // full size view works
        bitmap.window(0, 0, 8, 4).unwrap();

        // zero size view does not work
        assert!(Window::new(&mut bitmap, 1, 2, 3, 0).is_none());
        assert!(WindowMut::new(&mut bitmap, 1, 2, 0, 1).is_none());

        // oob does not work
        assert!(Window::new(&mut bitmap, 30, 43, 3, 1).is_none());
        assert!(WindowMut::new(&mut bitmap, 0, 0, 9, 1).is_none());
        assert!(Window::new(&mut bitmap, 0, 0, 1, 5).is_none());
    }

    #[test]
    fn test_grid_view_char_grid() {
        let mut grid = CharGrid::new(3, 4);
        grid.fill(' ');

        let mut view = WindowMut::new(&mut grid, 1, 1, 1, 3).unwrap();
        view.fill('#');
        view.set(0,0, '!');

        assert_eq!(
            grid.data_ref(),
            &[' ', ' ', ' ', ' ', '!', ' ', ' ', '#', ' ', ' ', '#', ' ']
        );

        // full size view works
        _ = Window::new(&mut grid, 0, 0, 3, 4).unwrap();

        // zero size view does not work
        assert!(Window::new(&mut grid, 1, 2, 2, 0).is_none());
        assert!(Window::new(&mut grid, 1, 2, 0, 1).is_none());
    }

    #[test]
    fn round_trip_bitmap() {
        let mut bitmap = Bitmap::new(8, 4).unwrap();

        let non_aligned = Window::new(&mut bitmap, 3, 1, 4, 2).unwrap();
        assert_eq!(Bitmap::try_from(&non_aligned), Err(()));

        let aligned = Window::new(&mut bitmap, 0, 1, 8, 2).unwrap();

        assert!(matches!(Bitmap::try_from(&aligned), Ok(_)));
    }
}
