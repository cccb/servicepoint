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
/// # use servicepoint::*;
/// let b = Brightness::MAX;
/// let val: u8 = b.into();
///
/// let b = Brightness::try_from(7).unwrap();
/// # let connection = FakeConnection;
/// let result = connection.send(BrightnessCommand::from(b));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Brightness(u8);

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

    /// Create a brightness value without returning an error for brightnesses above [`Brightness::MAX`].
    ///
    /// returns: the specified value as a [Brightness], or [`Brightness::MAX`].
    #[must_use]
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

#[cfg(feature = "rand")]
impl Distribution<Brightness> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Brightness {
        Brightness(rng.gen_range(Brightness::MIN.0..=Brightness::MAX.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let _: Brightness = rng.r#gen();
        }
    }

    #[test]
    fn saturating_convert() {
        assert_eq!(Brightness::MAX, Brightness::saturating_from(100));
        assert_eq!(Brightness(5), Brightness::saturating_from(5));
    }

    #[test]
    #[cfg(feature = "rand")]
    fn test() {
        let mut rng = rand::thread_rng();
        // two so test failure is less likely
        assert_ne!(
            [rng.r#gen::<Brightness>(), rng.r#gen()],
            [rng.r#gen(), rng.r#gen()]
        );
    }
}
