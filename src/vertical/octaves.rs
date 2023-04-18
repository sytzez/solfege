use crate::common::Scalar;
use crate::vertical::{InSemitones, InSteps, Semitones, Steps};
use std::ops::{Add, Sub};

/// Represents a distance in [octaves](https://en.wikipedia.org/wiki/Octave).
#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq, Hash, Debug)]
pub struct Octaves(pub Scalar);

pub trait InOctaves {
    /// Returns its distance in octaves
    fn in_octaves(&self) -> Octaves;
}

impl Add for Octaves {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Octaves {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl InSteps for Octaves {
    fn in_steps(&self) -> Steps {
        Steps(self.0 * 7)
    }
}

impl InSemitones for Octaves {
    fn in_semitones(&self) -> Semitones {
        Semitones(self.0 * 12)
    }
}

impl From<Scalar> for Octaves {
    fn from(scalar: Scalar) -> Self {
        Octaves(scalar)
    }
}

#[cfg(test)]
mod test {
    use crate::vertical::{InSemitones, InSteps, Semitones, Steps};

    use super::Octaves;

    #[test]
    fn in_steps() {
        assert_eq!(Octaves(3).in_steps(), Steps(21));
    }

    #[test]
    fn in_semitones() {
        assert_eq!(Octaves(3).in_semitones(), Semitones(36));
    }
}
