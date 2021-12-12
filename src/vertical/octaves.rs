use std::ops::{Add, Sub};
use crate::common::Scalar;
use crate::vertical::{AsSemitones, AsSteps, Semitones, Steps};

/// Represents a distance in [octaves](https://en.wikipedia.org/wiki/Octave)
#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq)]
pub struct Octaves(pub Scalar);

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

impl AsSteps for Octaves {
    fn as_steps(&self) -> Steps {
        Steps(self.0 * 7)
    }
}

impl AsSemitones for Octaves {
    fn as_semitones(&self) -> Semitones {
        Semitones(self.0 * 12)
    }
}
