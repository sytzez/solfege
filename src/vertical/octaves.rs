use std::ops::{Add, Sub};
use crate::common::Scalar;
use crate::vertical::{AsSemitones, AsSteps, Semitones, Steps};

#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq)]
pub struct Octaves(Scalar);

impl Octaves {
    pub fn new(value: Scalar) -> Self {
        Self(value)
    }
}

impl Add for Octaves {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
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
        Steps::new(self.0 * 7)
    }
}

impl AsSemitones for Octaves {
    fn as_semitones(&self) -> Semitones {
        Semitones::new(self.0 * 12)
    }
}
