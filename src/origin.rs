use crate::TILE_SIZE;
use std::marker::PhantomData;

/// An origin marks the top left position of a window sent to the display.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Origin<Unit: DisplayUnit> {
    /// position in the width direction
    pub x: usize,
    /// position in the height direction
    pub y: usize,
    phantom_data: PhantomData<Unit>,
}

impl<Unit: DisplayUnit> Origin<Unit> {
    /// Top-left. Equivalent to `Origin::ZERO`.
    pub const ZERO: Self = Self {
        x: 0,
        y: 0,
        phantom_data: PhantomData,
    };

    /// Create a new [Origin] instance for the provided position.
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            phantom_data: PhantomData,
        }
    }
}

impl<T: DisplayUnit> std::ops::Add<Origin<T>> for Origin<T> {
    type Output = Origin<T>;

    fn add(self, rhs: Origin<T>) -> Self::Output {
        Origin {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            phantom_data: PhantomData,
        }
    }
}

pub trait DisplayUnit {}

/// Marks something to be measured in number of pixels.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pixels();

/// Marks something to be measured in number of iles.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tiles();

impl DisplayUnit for Pixels {}

impl DisplayUnit for Tiles {}

impl From<&Origin<Tiles>> for Origin<Pixels> {
    fn from(value: &Origin<Tiles>) -> Self {
        Self {
            x: value.x * TILE_SIZE,
            y: value.y * TILE_SIZE,
            phantom_data: PhantomData,
        }
    }
}

impl<Unit: DisplayUnit> Default for Origin<Unit> {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_tile_to_pixel() {
        let tile: Origin<Tiles> = Origin::new(1, 2);
        let actual: Origin<Pixels> = Origin::from(&tile);
        let expected: Origin<Pixels> = Origin::new(8, 16);
        assert_eq!(actual, expected);
    }

    #[test]
    fn origin_add() {
        assert_eq!(
            Origin::<Pixels>::new(4, 2),
            Origin::new(1, 0) + Origin::new(3, 2)
        );
    }
}
