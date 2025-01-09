use crate::primitive_grid::PrimitiveGrid;
use crate::{ByteGrid, Grid};
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// A display brightness value, checked for correct value range
///
/// # Examples
///
/// ```
/// # use servicepoint::{Brightness, Command, Connection};
/// let b = Brightness::MAX;
/// let val: u8 = b.into();
///
/// let b = Brightness::try_from(7).unwrap();
/// # let connection = Connection::open("127.0.0.1:2342").unwrap();
/// let result = connection.send(Command::Brightness(b));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Brightness(u8);

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
pub type BrightnessGrid = PrimitiveGrid<Brightness>;

impl BrightnessGrid {
    /// Like [Self::load], but ignoring any out-of-range brightness values
    pub fn saturating_load(width: usize, height: usize, data: &[u8]) -> Self {
        PrimitiveGrid::load(width, height, data)
            .map(Brightness::saturating_from)
    }
}

impl From<Brightness> for u8 {
    fn from(brightness: Brightness) -> Self {
        Self::from(&brightness)
    }
}

impl From<&Brightness> for u8 {
    fn from(brightness: &Brightness) -> Self {
        brightness.0
    }
}

impl TryFrom<u8> for Brightness {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::MAX.0 {
            Err(value)
        } else {
            Ok(Brightness(value))
        }
    }
}

impl Brightness {
    /// highest possible brightness value, 11
    pub const MAX: Brightness = Brightness(11);
    /// lowest possible brightness value, 0
    pub const MIN: Brightness = Brightness(0);

    /// Create a brightness value without returning an error for brightnesses above [Brightness::MAX].
    ///
    /// returns: the specified value as a [Brightness], or [Brightness::MAX].
    pub fn saturating_from(value: u8) -> Brightness {
        if value > Brightness::MAX.into() {
            Brightness::MAX
        } else {
            Brightness(value)
        }
    }
}

impl Default for Brightness {
    fn default() -> Self {
        Self::MAX
    }
}

impl From<BrightnessGrid> for Vec<u8> {
    fn from(value: PrimitiveGrid<Brightness>) -> Self {
        value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect()
    }
}

impl From<&BrightnessGrid> for ByteGrid {
    fn from(value: &PrimitiveGrid<Brightness>) -> Self {
        let u8s = value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect::<Vec<u8>>();
        PrimitiveGrid::load(value.width(), value.height(), &u8s)
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

#[cfg(feature = "rand")]
impl Distribution<Brightness> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Brightness {
        Brightness(rng.gen_range(Brightness::MIN.0..=Brightness::MAX.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataRef;

    #[test]
    fn brightness_from_u8() {
        assert_eq!(Err(100), Brightness::try_from(100));
        assert_eq!(Ok(Brightness(1)), Brightness::try_from(1));
    }

    #[test]
    #[cfg(feature = "rand")]
    fn rand_brightness() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let _: Brightness = rng.gen();
        }
    }

    #[test]
    fn to_u8_grid() {
        let mut grid = BrightnessGrid::new(2, 2);
        grid.set(1, 0, Brightness::MIN);
        grid.set(0, 1, Brightness::MAX);
        let actual = PrimitiveGrid::from(&grid);
        assert_eq!(actual.data_ref(), &[11, 0, 11, 11]);
    }

    #[test]
    fn saturating_convert() {
        assert_eq!(Brightness::MAX, Brightness::saturating_from(100));
        assert_eq!(Brightness(5), Brightness::saturating_from(5));
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
