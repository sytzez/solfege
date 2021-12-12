use std::ops::{Add, Sub};
use crate::common::Scalar;

/// Represents a distance in [semitones](https://en.wikipedia.org/wiki/Semitone).
#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq)]
pub struct Semitones(pub Scalar);

pub trait AsSemitones {
    /// Returns its distance in semitones;
    fn as_semitones(&self) -> Semitones;
}

pub trait SemitonesFromC {
    /// Returns the relative position in semitones upward from C
    fn semitones_from_c(&self) -> Semitones;
}

pub trait SemitonesFromC0 {
    /// Returns the relative position in semitones from C0
    fn semitones_from_c0(&self) -> Semitones;
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
