use std::ops::{Add, Sub};
use crate::common::Scalar;

#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq)]
pub struct Steps(pub Scalar);

pub trait AsSteps {
    fn as_steps(&self) -> Steps;
}

pub trait StepsFromC {
    fn steps_from_c(&self) -> Steps;
}

pub trait StepsFromC0 {
    fn steps_from_c0(&self) -> Steps;
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
