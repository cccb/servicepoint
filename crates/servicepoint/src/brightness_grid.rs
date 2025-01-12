use crate::brightness::Brightness;
use crate::grid::Grid;
use crate::value_grid::ValueGrid;
use crate::ByteGrid;

/// A grid containing brightness values.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Brightness, BrightnessGrid, Command, Connection, Grid, Origin};
/// let mut grid = BrightnessGrid::new(2,2);
/// grid.set(0, 0, Brightness::MIN);
/// grid.set(1, 1, Brightness::MIN);
///
/// # let connection = Connection::open("127.0.0.1:2342").unwrap();
/// connection.send(Command::CharBrightness(Origin::new(3, 7), grid)).unwrap()
/// ```
pub type BrightnessGrid = ValueGrid<Brightness>;

impl BrightnessGrid {
    /// Like [Self::load], but ignoring any out-of-range brightness values
    pub fn saturating_load(width: usize, height: usize, data: &[u8]) -> Self {
        ValueGrid::load(width, height, data).map(Brightness::saturating_from)
    }
}

impl From<BrightnessGrid> for Vec<u8> {
    fn from(value: ValueGrid<Brightness>) -> Self {
        value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect()
    }
}

impl From<&BrightnessGrid> for ByteGrid {
    fn from(value: &ValueGrid<Brightness>) -> Self {
        let u8s = value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect::<Vec<u8>>();
        ValueGrid::load(value.width(), value.height(), &u8s)
    }
}

impl TryFrom<ByteGrid> for BrightnessGrid {
    type Error = u8;

    fn try_from(value: ByteGrid) -> Result<Self, Self::Error> {
        let brightnesses = value
            .iter()
            .map(|b| Brightness::try_from(*b))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BrightnessGrid::load(
            value.width(),
            value.height(),
            &brightnesses,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::value_grid::ValueGrid;
    use crate::{Brightness, BrightnessGrid, DataRef, Grid};

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
            ),
            BrightnessGrid::saturating_load(2, 2, &[255u8, 23, 0, 42])
        );
    }
}
