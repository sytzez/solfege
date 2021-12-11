use std::ops::{Add, Sub};
use crate::common::Scalar;

#[derive(Copy, Eq, Ord)]
pub struct Steps(Scalar);

pub trait AsSteps {
    fn as_steps(&self) -> Steps;
}

pub trait StepsFromC {
    fn steps_from_c(&self) -> Steps;
}

pub trait StepsFromC0 {
    fn steps_from_c0(&self) -> Steps;
}

impl Steps {
    pub fn new(value: Scalar) -> Self {
        Self(value)
    }
}

impl Add for Steps {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Steps {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
