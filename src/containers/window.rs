use crate::{
    containers::char_grid::{CharGridExt, CharGridMutExt},
    Grid, GridMut,
};
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

            /// Creates a window into the window.
            ///
            /// Returns None in case the window does not fit.
            pub fn window(
                &self,
                x: usize,
                y: usize,
                width: usize,
                height: usize,
            ) -> Option<Window<TElement, TGrid>> {
                if x + width >= self.width || y + height >= self.height {
                    return None;
                }
                Window::new(self.grid, self.x + x, self.y + y, width, height)
            }

            pub fn split_horizontal(
                self,
                left_width: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                let left = Window::new(self.grid, 0, 0, left_width, self.height)?;
                let right = Window::new(
                    self.grid,
                    left_width,
                    0,
                    self.width - left_width,
                    self.height,
                )?;
                Some((left, right))
            }

            pub fn split_vertical(
                self,
                top_height: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                let top = Window::new(self.grid, 0, 0, self.width, top_height)?;
                let bottom = Window::new(
                    self.grid,
                    0,
                    top_height,
                    self.width,
                    self.height - top_height,
                )?;
                Some((top, bottom))
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

        #[inherent::inherent]
        impl<TGrid: Grid<char>> CharGridExt for $name<'_, char, TGrid> {}
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

#[inherent::inherent]
impl<TGrid: GridMut<char>> CharGridMutExt for WindowMut<'_, char, TGrid> {}

impl<'t, TElement: Copy, TGrid: GridMut<TElement>>
    WindowMut<'t, TElement, TGrid>
{
    /// Creates a mutable window into the grid.
    ///
    /// Returns None in case the window does not fit.
    pub fn window_mut(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Option<WindowMut<TElement, TGrid>> {
        if x + width >= self.width || y + height >= self.height {
            return None;
        }
        WindowMut::new(self.grid, self.x + x, self.y + y, width, height)
    }

    pub fn split_horizontal_mut(
        self,
        left_width: usize,
    ) -> Option<(
        WindowMut<'t, TElement, TGrid>,
        WindowMut<'t, TElement, TGrid>,
    )> {
        let (grid1, grid2) = unsafe { Self::duplicate_mutable_ref(self.grid) };
        let left = WindowMut::new(grid1, 0, 0, left_width, self.height)?;
        let right =
            WindowMut::new(grid2, left_width, 0, self.width - left_width, self.height)?;
        Some((left, right))
    }

    pub fn split_vertical_mut(
        self,
        top_height: usize,
    ) -> Option<(
        WindowMut<'t, TElement, TGrid>,
        WindowMut<'t, TElement, TGrid>,
    )> {
        let (grid1, grid2) = unsafe { Self::duplicate_mutable_ref(self.grid) };
        let top = WindowMut::new(grid1, 0, 0, self.width, top_height)?;
        let bottom =
            WindowMut::new(grid2, 0, top_height, self.width, self.height - top_height)?;
        Some((top, bottom))
    }

    /// SAFETY: the returned windows do not alias
    /// Does not work if the grid uses the same memory location for multiple cells internally.
    /// That means for e.g. a Bitmap, middle must be byte aligned or bit refs only used with the alias flag on.
    unsafe fn duplicate_mutable_ref<T>(it: &mut T) -> (&mut T, &mut T) {
        let mut ptr = std::ptr::NonNull::from(it);
        unsafe { (ptr.as_mut(), ptr.as_mut()) }
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
        view.set(0, 0, '!');

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
