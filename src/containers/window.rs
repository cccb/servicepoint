use crate::{
    containers::{absolute_bounds_to_abs_range, relative_bounds_to_abs_range},
    Grid, GridMut,
};
use std::{
    marker::PhantomData,
    ops::{Range, RangeBounds},
};

macro_rules! define_window {
    ($name:ident, $grid:ty) => {
        /// A window into a 2D grid.
        ///
        /// All operations are done directly on the contained reference,
        /// but translated to where the window is.
        #[derive(Debug)]
        pub struct $name<'t, TElement: Copy, TGrid: Grid<TElement>> {
            grid: $grid,
            xs: Range<usize>,
            ys: Range<usize>,
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
                xs: impl RangeBounds<usize>,
                ys: impl RangeBounds<usize>,
            ) -> Option<Self> {
                let xs = absolute_bounds_to_abs_range(xs, grid.width())?;
                let ys = absolute_bounds_to_abs_range(ys, grid.height())?;
                Some(Self {
                    grid,
                    xs,
                    ys,
                    phantom: PhantomData::default(),
                })
            }

            #[must_use]
            /// Creates a window into the window.
            ///
            /// Returns None in case the window does not fit.
            pub fn window(
                &self,
                xs: impl RangeBounds<usize>,
                ys: impl RangeBounds<usize>,
            ) -> Option<Window<TElement, TGrid>> {
                let xs = relative_bounds_to_abs_range(xs, self.xs.clone())?;
                let ys = relative_bounds_to_abs_range(ys, self.ys.clone())?;
                Window::new(self.grid, xs, ys)
            }

            /// Splits the window horizontally, returning windows to the left and right parts.
            ///
            /// The right window fills the remaining width, which may be zero.
            ///
            /// Returns None for out-of-bounds.
            #[must_use]
            pub fn split_horizontal(
                &'t self,
                left_width: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                assert!(left_width <= self.width());
                let middle_abs = self.xs.start + left_width;
                let left = Window::new(
                    self.grid,
                    self.xs.start..middle_abs,
                    self.ys.clone(),
                )?;
                let right = Window::new(
                    self.grid,
                    middle_abs..self.xs.end,
                    self.ys.clone(),
                )?;
                Some((left, right))
            }

            /// Splits the window vertically, returning windows to the top and bottom parts.
            ///
            /// The bottom window fills the remaining height, which may be zero.
            ///
            /// Returns None for out-of-bounds.
            #[must_use]
            pub fn split_vertical(
                &'t self,
                top_height: usize,
            ) -> Option<(
                Window<'t, TElement, TGrid>,
                Window<'t, TElement, TGrid>,
            )> {
                assert!(top_height <= self.height());
                let middle_abs = self.ys.start + top_height;
                let top = Window::new(
                    self.grid,
                    self.xs.clone(),
                    self.ys.start..middle_abs,
                )?;
                let bottom = Window::new(
                    self.grid,
                    self.xs.clone(),
                    middle_abs..self.ys.end,
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
                self.grid.get(self.xs.start + x, self.ys.start + y)
            }

            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn width(&self) -> usize {
                self.xs.len()
            }

            #[must_use]
            #[allow(unused, reason = "False positive because of #[inherent]")]
            pub fn height(&self) -> usize {
                self.ys.len()
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
        self.grid.set(self.xs.start + x, self.ys.start + y, value);
    }

    #[allow(unused, reason = "False positive because of #[inherent]")]
    pub fn fill(&mut self, value: TElement) {
        for y in self.ys.clone() {
            for x in self.xs.clone() {
                self.grid.set(x, y, value);
            }
        }
    }
}

impl<TElement: Copy, TGrid: GridMut<TElement>> WindowMut<'_, TElement, TGrid> {
    /// Creates a mutable window into the grid.
    ///
    /// Returns None in case the window does not fit.
    pub fn window_mut(
        &mut self,
        xs: impl RangeBounds<usize>,
        ys: impl RangeBounds<usize>,
    ) -> Option<WindowMut<TElement, TGrid>> {
        let xs = relative_bounds_to_abs_range(xs, self.xs.clone())?;
        let ys = relative_bounds_to_abs_range(ys, self.ys.clone())?;
        WindowMut::new(self.grid, xs, ys)
    }

    /// Splits the window horizontally, returning windows to the left and right parts.
    ///
    /// The right window fills the remaining width, which may be zero.
    ///
    /// Returns None for out-of-bounds.
    #[must_use]
    pub fn split_horizontal_mut<'t>(
        &'t mut self,
        left_width: usize,
    ) -> Option<(
        WindowMut<'t, TElement, TGrid>,
        WindowMut<'t, TElement, TGrid>,
    )> {
        assert!(left_width <= self.width());
        let (grid1, grid2): (&'t mut TGrid, &'t mut TGrid) =
            unsafe { Self::duplicate_mutable_ref(self.grid) };
        let middle_abs = self.xs.start + left_width;
        let left =
            WindowMut::new(grid1, self.xs.start..middle_abs, self.ys.clone())?;
        let right =
            WindowMut::new(grid2, middle_abs..self.xs.end, self.ys.clone())?;
        Some((left, right))
    }

    /// Splits the window vertically, returning windows to the top and bottom parts.
    ///
    /// The bottom window fills the remaining height, which may be zero.
    ///
    /// Returns None for out-of-bounds.
    #[must_use]
    pub fn split_vertical_mut<'t>(
        &'t mut self,
        top_height: usize,
    ) -> Option<(
        WindowMut<'t, TElement, TGrid>,
        WindowMut<'t, TElement, TGrid>,
    )> {
        assert!(top_height <= self.height());
        let (grid1, grid2): (&'t mut TGrid, &'t mut TGrid) =
            unsafe { Self::duplicate_mutable_ref(self.grid) };
        let middle_abs = self.ys.start + top_height;
        let top = WindowMut::<'t>::new(
            grid1,
            self.xs.clone(),
            self.ys.start..middle_abs,
        )?;
        let bottom = WindowMut::<'t>::new(
            grid2,
            self.xs.clone(),
            middle_abs..self.ys.end,
        )?;
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
    use crate::{Bitmap, ByteGrid, CharGrid, DataRef, Grid, GridMut};

    #[test]
    fn grid_view_bitmap() {
        let mut bitmap = Bitmap::new(8, 4).unwrap();

        // non-byte-aligned views work
        let mut view = bitmap.window_mut(3..7, 1..3).unwrap();
        view.fill(true);

        assert_eq!(bitmap.data_ref(), &[0, 30, 30, 0]);

        assert_eq!(bitmap.set_optional(99, 99, false), false);

        // full size view works
        bitmap.window(0..8, 0..4).unwrap();

        // zero size view works
        assert!(Window::new(&mut bitmap, 1..4, 2..2).is_some());
        assert!(WindowMut::new(&mut bitmap, 1..1, 2..3)
            .is_some_and(|w| w.get_optional(0, 0).is_none()));

        // oob does not work
        assert!(Window::new(&mut bitmap, 30..33, 43..44).is_none());
        assert!(WindowMut::new(&mut bitmap, 0..9, 0..1).is_none());
        assert!(Window::new(&mut bitmap, 0..1, 0..5).is_none());
    }

    #[test]
    fn grid_view_char_grid() {
        let mut grid = CharGrid::new(3, 4);
        grid.fill(' ');

        let mut view = grid.window_mut(1..2, 1..4).unwrap();
        view.fill('#');
        view.set(0, 0, '!');

        assert_eq!(
            grid.data_ref(),
            &[' ', ' ', ' ', ' ', '!', ' ', ' ', '#', ' ', ' ', '#', ' ']
        );

        // full size view works
        _ = grid.window(0..3, 0..4).unwrap();

        // zero size view works
        assert!(grid
            .window(1..3, 2..2)
            .is_some_and(|w| w.get_optional(0, 0).is_none()));
        assert!(grid.window(1..1, 2..3).is_some());
    }

    #[test]
    fn round_trip_bitmap() {
        let bitmap = Bitmap::new(8, 4).unwrap();

        let non_aligned = bitmap.window(3..7, 1..3).unwrap();
        assert_eq!(Bitmap::try_from(&non_aligned), Err(()));

        let aligned = bitmap.window(0..8, 1..3).unwrap();
        assert!(matches!(Bitmap::try_from(&aligned), Ok(_)));
    }

    #[test]
    fn split_vertical() {
        let grid = ByteGrid::new(5, 4);
        let window = grid.window(0..grid.width(), 0..grid.height()).unwrap();

        let (left, right) = window.split_vertical(3).unwrap();
        assert_eq!(3, left.height());
        assert_eq!(1, right.height());
        assert_eq!(5, left.width());
        assert_eq!(5, right.width())
    }

    #[test]
    fn split_horizontal() {
        let grid = ByteGrid::new(4, 5);
        let window = grid.window(.., ..).unwrap();

        let (top, bottom) = window.split_horizontal(3).unwrap();
        assert_eq!(3, top.width());
        assert_eq!(1, bottom.width());
        assert_eq!(5, top.height());
        assert_eq!(5, bottom.height())
    }

    #[test]
    fn window_in_window() {
        let mut grid = ByteGrid::new(6, 7);
        grid.fill(1);

        let mut w1 = grid
            .window_mut(1..grid.width() - 1, 1..grid.height() - 1)
            .unwrap();
        w1.fill(2);

        let w1_1 = w1.window(.., ..).unwrap();
        assert_eq!(w1_1.get(0, 0), 2);

        assert!(matches!(w1.window(.., 0..=w1.height()), None));

        let mut w2 = w1
            .window_mut(1..w1.width() - 1, 1..w1.height() - 1)
            .unwrap();
        w2.fill(3);

        // zero-sized
        let mut w3 = w2
            .window_mut(1..w2.width() - 1, 1..w2.height() - 1)
            .unwrap();
        w3.fill(4);

        #[rustfmt::skip]
        assert_eq!(
            grid.data_ref(),
            &[
                1, 1, 1, 1, 1, 1,
                1, 2, 2, 2, 2, 1,
                1, 2, 3, 3, 2, 1,
                1, 2, 3, 3, 2, 1,
                1, 2, 3, 3, 2, 1,
                1, 2, 2, 2, 2, 1,
                1, 1, 1, 1, 1, 1
            ]
        );
    }

    #[test]
    fn width_height() {
        let grid = ByteGrid::new(4, 4);
        let w1 = grid.window(0.., ..grid.height()).unwrap();
        assert_eq!(grid.width(), w1.width());
        assert_eq!(grid.height(), w1.height());
        let w2 = w1.window(.., 0..w1.height()).unwrap();
        assert_eq!(grid.width(), w2.width());
        assert_eq!(grid.height(), w2.height());
    }

    #[test]
    fn split_mut() {
        let mut grid = ByteGrid::new(5, 5);
        grid.fill(1);

        let mut win = grid.window_mut(.., ..).unwrap();
        let (mut top, mut bottom) = win.split_vertical_mut(2).unwrap();
        let (mut left, mut right) = bottom.split_horizontal_mut(2).unwrap();

        top.fill(2);
        left.fill(3);
        right.fill(4);

        let grid2 = ByteGrid::from(&win.window(1..4, 1..4).unwrap());

        assert_eq!(grid2.data_ref(), &[2, 2, 2, 3, 4, 4, 3, 4, 4,]);
        assert_eq!(
            grid.data_ref(),
            &[
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 4, 4, 4, 3, 3, 4, 4, 4, 3,
                3, 4, 4, 4,
            ]
        );
    }
}
