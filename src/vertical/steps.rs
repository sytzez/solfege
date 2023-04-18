use crate::common::Scalar;
use crate::vertical::{InOctaves, Octaves};
use std::ops::{Add, Sub};

/// Represents a distance in [steps](https://en.wikipedia.org/wiki/Steps_and_skips).
#[derive(Copy, Eq, Ord, Clone, PartialOrd, PartialEq, Hash, Debug)]
pub struct Steps(pub Scalar);

pub trait InSteps {
    /// Returns its distance in steps.
    fn in_steps(&self) -> Steps;
}

pub trait StepsFromC {
    /// Returns the relative position in steps upward from C.
    fn steps_from_c(&self) -> Steps;
}

pub trait StepsFromC0 {
    /// Returns the relative position in steps from C0.
    fn steps_from_c0(&self) -> Steps;
}

impl InOctaves for Steps {
    fn in_octaves(&self) -> Octaves {
        return Octaves(self.0 / 7);
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

impl From<Scalar> for Steps {
    fn from(scalar: Scalar) -> Self {
        Steps(scalar)
    }
}

#[cfg(test)]
mod test {
    use crate::vertical::{InOctaves, Octaves};

    use super::Steps;

    #[test]
    fn in_octaves() {
        assert_eq!(Steps(6).in_octaves(), Octaves(0));
        assert_eq!(Steps(7).in_octaves(), Octaves(1));
        assert_eq!(Steps(13).in_octaves(), Octaves(1));
        assert_eq!(Steps(15).in_octaves(), Octaves(2));
    }
}
