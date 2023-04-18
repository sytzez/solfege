use crate::common::Scalar;
use crate::vertical::{InOctaves, Octaves};
use std::ops::{Add, Neg, Sub};

/// Represents a distance in [semitones](https://en.wikipedia.org/wiki/Semitone).
#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq, Hash, Debug)]
pub struct Semitones(pub Scalar);

pub trait InSemitones {
    /// Returns its distance in semitones.
    fn in_semitones(&self) -> Semitones;
}

pub trait SemitonesFromC {
    /// Returns the relative position in semitones upward from C.
    fn semitones_from_c(&self) -> Semitones;
}

pub trait SemitonesFromC0 {
    /// Returns the relative position in semitones from C0.
    fn semitones_from_c0(&self) -> Semitones;
}

impl InOctaves for Semitones {
    fn in_octaves(&self) -> Octaves {
        return Octaves(self.0 / 12);
    }
}

impl Add for Semitones {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Semitones {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Semitones {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Semitones(-self.0)
    }
}

impl From<Scalar> for Semitones {
    fn from(scalar: Scalar) -> Self {
        Semitones(scalar)
    }
}

#[cfg(test)]
mod test {
    use crate::vertical::{InOctaves, Octaves};

    use super::Semitones;

    #[test]
    fn in_octaves() {
        assert_eq!(Semitones(11).in_octaves(), Octaves(0));
        assert_eq!(Semitones(12).in_octaves(), Octaves(1));
        assert_eq!(Semitones(23).in_octaves(), Octaves(1));
        assert_eq!(Semitones(24).in_octaves(), Octaves(2));
    }
}
