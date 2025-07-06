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

            #[must_use]
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

            #[must_use]
            pub fn split_horizontal(
                self,
                left_width: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                let left = Window::new(
                    self.grid,
                    self.x,
                    self.y,
                    left_width,
                    self.height,
                )?;
                let right = Window::new(
                    self.grid,
                    self.x + left_width,
                    self.y,
                    self.width - left_width,
                    self.height,
                )?;
                Some((left, right))
            }

            #[must_use]
            pub fn split_vertical(
                self,
                top_height: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                let top = Window::new(
                    self.grid, self.x, self.y, self.width, top_height,
                )?;
                let bottom = Window::new(
                    self.grid,
                    self.x,
                    self.y + top_height,
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

impl<TElement: Copy, TGrid: GridMut<TElement>>
    WindowMut<'_, TElement, TGrid>
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
        if x + width > self.width || y + height > self.height {
            return None;
        }
        WindowMut::new(self.grid, self.x + x, self.y + y, width, height)
    }

    pub fn deref_assign<O: Grid<TElement>>(&mut self, other: &O) {
        assert!(self.width() == other.width());
        assert!(self.height() == other.height());
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, other.get(x, y));
            }
        }
    }

    #[must_use]
    pub fn split_horizontal_mut(
        self,
        left_width: usize,
    ) -> Option<(Self, Self)> {
        let (grid1, grid2) = unsafe { Self::duplicate_mutable_ref(self.grid) };
        let left =
            WindowMut::new(grid1, self.x, self.y, left_width, self.height)?;
        let right = WindowMut::new(
            grid2,
            self.x + left_width,
            self.y,
            self.width - left_width,
            self.height,
        )?;
        Some((left, right))
    }

    #[must_use]
    pub fn split_vertical_mut(self, top_height: usize) -> Option<(Self, Self)> {
        let (grid1, grid2) = unsafe { Self::duplicate_mutable_ref(self.grid) };
        let top =
            WindowMut::new(grid1, self.x, self.y, self.width, top_height)?;
        let bottom = WindowMut::new(
            grid2,
            self.x,
            self.y + top_height,
            self.width,
            self.height - top_height,
        )?;

        let foo = &mut [..];
        *foo = [..];

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
    use super::{Window, WindowMut};
    use crate::{Bitmap, ByteGrid, CharGrid, DataRef, GridMut};

    #[test]
    fn grid_view_bitmap() {
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
    fn grid_view_char_grid() {
        let mut grid = CharGrid::new(3, 4);
        grid.fill(' ');

        let mut view = grid.window_mut(1, 1, 1, 3).unwrap();
        view.fill('#');
        view.set(0, 0, '!');

        assert_eq!(
            grid.data_ref(),
            &[' ', ' ', ' ', ' ', '!', ' ', ' ', '#', ' ', ' ', '#', ' ']
        );

        // full size view works
        _ = grid.window(0, 0, 3, 4).unwrap();

        // zero size view does not work
        assert!(grid.window(1, 2, 2, 0).is_none());
        assert!(grid.window(1, 2, 0, 1).is_none());
    }

    #[test]
    fn round_trip_bitmap() {
        let bitmap = Bitmap::new(8, 4).unwrap();

        let non_aligned = bitmap.window(3, 1, 4, 2).unwrap();
        assert_eq!(Bitmap::try_from(&non_aligned), Err(()));

        let aligned = bitmap.window(0, 1, 8, 2).unwrap();
        assert!(matches!(Bitmap::try_from(&aligned), Ok(_)));
    }

    #[test]
    fn split_vertical() {
        let grid = ByteGrid::new(5, 4);
        let window = grid.window(0, 0, grid.width(), grid.height()).unwrap();

        let (left, right) = window.split_vertical(3).unwrap();
        assert_eq!(3, left.height());
        assert_eq!(1, right.height());
        assert_eq!(5, left.width());
        assert_eq!(5, right.width())
    }

    #[test]
    fn split_horizontal() {
        let grid = ByteGrid::new(4, 5);
        let window = grid.window(0, 0, grid.width(), grid.height()).unwrap();

        let (top, bottom) = window.split_horizontal(3).unwrap();
        assert_eq!(3, top.width());
        assert_eq!(1, bottom.width());
        assert_eq!(5, top.height());
        assert_eq!(5, bottom.height())
    }
}
