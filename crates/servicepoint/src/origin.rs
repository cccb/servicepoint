use std::marker::PhantomData;

/// An origin marks the top left position of a window sent to the display.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Origin<Unit: DisplayUnit> {
    /// position in the width direction
    pub x: usize,
    /// position in the height direction
    pub y: usize,
    phantom_data: PhantomData<Unit>,
}

impl<Unit: DisplayUnit> Origin<Unit> {
    /// Create a new `Origin` instance for the provided position.
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            phantom_data: PhantomData::default(),
        }
    }
}

impl<T: DisplayUnit> std::ops::Add<Origin<T>> for Origin<T> {
    type Output = Origin<T>;

    fn add(self, rhs: Origin<T>) -> Self::Output {
        Origin {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            phantom_data: PhantomData::default(),
        }
    }
}

pub trait DisplayUnit {}

/// Marks something to be measured in number of pixels.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixels();

/// Marks something to be measured in number of iles.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tiles();

impl DisplayUnit for Pixels {}

impl DisplayUnit for Tiles {}
