use std::ops::{Add, Sub};
use crate::common::Scalar;

#[derive(Copy, Eq, Ord)]
pub struct Semitones(Scalar);

pub trait AsSemitones {
    fn as_semitones(&self) -> Semitones;
}

pub trait SemitonesFromC {
    fn semitones_from_c(&self) -> Semitones;
}

pub trait SemitonesFromC0 {
    fn semitones_from_c0(&self) -> Semitones;
}

impl Semitones {
    pub fn new(value: Scalar) -> Self {
        Self(value)
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
