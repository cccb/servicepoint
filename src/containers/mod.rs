mod bit_vec;
mod bitmap;
mod brightness_grid;
mod byte_grid;
mod char_grid;
mod char_grid_ext;
mod cp437_grid;
mod data_ref;
mod grid;
mod value_grid;
mod window;

pub use bit_vec::{bitvec, DisplayBitVec};
pub use bitmap::{Bitmap, LoadBitmapError};
pub use brightness_grid::BrightnessGrid;
pub use byte_grid::ByteGrid;
pub use char_grid::CharGrid;
pub use char_grid_ext::{CharGridExt, CharGridMutExt};
pub use cp437_grid::{Cp437Grid, InvalidCharError};
pub use data_ref::DataRef;
pub use grid::{Grid, GridMut};
pub use value_grid::{
    SetValueSeriesError, TryLoadValueGridError, Value, ValueGrid,
};
pub use window::{Window, WindowMut};

use std::{
    collections::Bound,
    ops::{Range, RangeBounds},
};

pub(crate) fn absolute_bounds_to_abs_range(
    bounds: impl RangeBounds<usize>,
    len: usize,
) -> Option<Range<usize>> {
    let start = match bounds.start_bound() {
        Bound::Included(start) => *start,
        Bound::Excluded(start) => start + 1,
        Bound::Unbounded => 0,
    };

    let end = match bounds.end_bound() {
        Bound::Included(end) => end + 1,
        Bound::Excluded(end) => *end,
        Bound::Unbounded => len,
    };
    if end > len {
        return None;
    }

    Some(start..end)
}

pub(crate) fn relative_bounds_to_abs_range(
    bounds: impl RangeBounds<usize>,
    range: Range<usize>,
) -> Option<Range<usize>> {
    let relative = absolute_bounds_to_abs_range(bounds, range.len())?;
    let start = range.start + relative.start;
    let end = range.start + relative.end;
    if end > range.end {
        return None;
    }
    Some(start..end)
}
