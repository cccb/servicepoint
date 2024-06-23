#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/// A display brightness value, checked for correct value range
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Brightness(u8);

impl From<Brightness> for u8 {
    fn from(brightness: Brightness) -> Self {
        brightness.0
    }
}

impl TryFrom<u8> for Brightness {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::MAX.0 {
            Err(())
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

#[cfg(feature = "rand")]
impl Distribution<Brightness> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Brightness {
        Brightness(rng.gen_range(Brightness::MIN.0..(Brightness::MAX.0 + 1)))
    }
}
