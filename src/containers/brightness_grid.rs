use crate::{Brightness, ByteGrid, Grid, ValueGrid};

/// A grid containing brightness values.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// let mut grid = BrightnessGrid::new(2,2);
/// grid.set(0, 0, Brightness::MIN);
/// grid.set(1, 1, Brightness::MIN);
///
/// # let connection = FakeConnection;
/// connection.send_command(BrightnessGridCommand {
///     origin: Origin::new(3, 7),
///     grid
/// }).unwrap()
/// ```
pub type BrightnessGrid = ValueGrid<Brightness>;

impl BrightnessGrid {
    /// Like [`Self::load`], but ignoring any out-of-range brightness values
    #[must_use]
    pub fn saturating_load(
        width: usize,
        height: usize,
        data: &[u8],
    ) -> Option<Self> {
        ValueGrid::load(width, height, data)
            .map(move |grid| grid.map(Brightness::saturating_from))
    }
}

impl From<&BrightnessGrid> for Vec<u8> {
    fn from(value: &BrightnessGrid) -> Self {
        value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect()
    }
}

impl From<BrightnessGrid> for Vec<u8> {
    fn from(value: BrightnessGrid) -> Self {
        // look mom, zero copies!

        let mut vec =
            std::mem::ManuallyDrop::new(Vec::<Brightness>::from(value));

        // Safety: this is safe because Brightness is repr(transparent) and only contains one u8.
        // Also see https://doc.rust-lang.org/std/mem/fn.transmute.html
        unsafe {
            // this makes sure the operation is safe at compile time
            const _: () = assert!(
                size_of::<Brightness>() == size_of::<u8>()
                    && align_of::<Brightness>() == align_of::<u8>()
            );

            Vec::from_raw_parts(
                vec.as_mut_ptr().cast(),
                vec.len(),
                vec.capacity(),
            )
        }
    }
}

impl From<&BrightnessGrid> for ByteGrid {
    fn from(value: &BrightnessGrid) -> Self {
        let u8s = value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect::<Vec<u8>>();
        Self::from_raw_parts_unchecked(value.width(), value.height(), u8s)
    }
}

impl TryFrom<ByteGrid> for BrightnessGrid {
    type Error = u8;

    fn try_from(value: ByteGrid) -> Result<Self, Self::Error> {
        let brightnesses = value
            .iter()
            .map(|b| Brightness::try_from(*b))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::from_raw_parts_unchecked(
            value.width(),
            value.height(),
            brightnesses,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Brightness, BrightnessGrid, DataRef, Grid, ValueGrid};

    #[test]
    fn to_u8_grid() {
        let mut grid = BrightnessGrid::new(2, 2);
        grid.set(1, 0, Brightness::MIN);
        grid.set(0, 1, Brightness::MAX);
        let actual = ValueGrid::from(&grid);
        assert_eq!(actual.data_ref(), &[11, 0, 11, 11]);
    }

    #[test]
    fn saturating_load() {
        assert_eq!(
            BrightnessGrid::load(
                2,
                2,
                &[
                    Brightness::MAX,
                    Brightness::MAX,
                    Brightness::MIN,
                    Brightness::MAX
                ]
            )
            .unwrap(),
            BrightnessGrid::saturating_load(2, 2, &[255u8, 23, 0, 42]).unwrap()
        );
    }
}
