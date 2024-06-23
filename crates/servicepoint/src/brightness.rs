use crate::{Grid, PrimitiveGrid};
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// A display brightness value, checked for correct value range
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Brightness(u8);

/// A grid containing brightness values.
pub type BrightnessGrid = PrimitiveGrid<Brightness>;

impl From<Brightness> for u8 {
    fn from(brightness: Brightness) -> Self {
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

impl From<BrightnessGrid> for PrimitiveGrid<u8> {
    fn from(value: PrimitiveGrid<Brightness>) -> Self {
        let u8s = value
            .iter()
            .map(|brightness| (*brightness).into())
            .collect::<Vec<u8>>();
        PrimitiveGrid::load(value.width(), value.height(), &u8s)
    }
}

impl TryFrom<PrimitiveGrid<u8>> for BrightnessGrid {
    type Error = u8;

    fn try_from(value: PrimitiveGrid<u8>) -> Result<Self, Self::Error> {
        let brightnesses = value
            .iter()
            .map(|b| Brightness::try_from(*b))
            .collect::<Result<Vec<Brightness>, _>>();
        let brightnesses = match brightnesses {
            Ok(vec) => vec,
            Err(u8) => return Err(u8),
        };
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
        Brightness(rng.gen_range(Brightness::MIN.0..(Brightness::MAX.0 + 1)))
    }
}
