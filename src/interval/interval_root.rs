use crate::interval::IntervalQuality::{Imperfect, Perfectable};
use crate::interval::{GetIntervalQuality, IntervalQuality, Inverted, IsPerfect};
use crate::pitch::PitchRootDyad;
use crate::vertical::{InSemitones, InSteps, Semitones, Steps, StepsFromC};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// One of the 7 [simple intervals](https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals): Unison, Second, Third, Fourth, Fifth, Sixth or Seventh.
///
/// # Examples
/// ```
/// use solfege::interval::IntervalRoot;
///
/// let third = IntervalRoot::Third;
///
/// assert_eq!(third.to_string(), "3");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum IntervalRoot {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl InSteps for IntervalRoot {
    fn in_steps(&self) -> Steps {
        let value = match *self {
            Self::Unison => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Fifth => 4,
            Self::Sixth => 5,
            Self::Seventh => 6,
        };

        Steps(value)
    }
}

impl InSemitones for IntervalRoot {
    fn in_semitones(&self) -> Semitones {
        let value = match *self {
            Self::Unison => 0,
            Self::Second => 2,
            Self::Third => 4,
            Self::Fourth => 5,
            Self::Fifth => 7,
            Self::Sixth => 9,
            Self::Seventh => 11,
        };

        Semitones(value)
    }
}

impl IsPerfect for IntervalRoot {
    fn is_perfect(&self) -> bool {
        match *self {
            Self::Unison => true,
            Self::Second => false,
            Self::Third => false,
            Self::Fourth => true,
            Self::Fifth => true,
            Self::Sixth => false,
            Self::Seventh => false,
        }
    }
}

impl GetIntervalQuality for IntervalRoot {
    fn get_interval_quality(&self) -> IntervalQuality {
        match self.is_perfect() {
            true => Perfectable(Semitones(0)),
            false => Imperfect(Semitones(0)),
        }
    }
}

impl Add for IntervalRoot {
    type Output = IntervalRoot;

    fn add(self, rhs: Self) -> Self::Output {
        IntervalRoot::from(self.in_steps() + rhs.in_steps())
    }
}

impl Sub for IntervalRoot {
    type Output = IntervalRoot;

    fn sub(self, rhs: Self) -> Self::Output {
        IntervalRoot::from(self.in_steps() - rhs.in_steps())
    }
}

impl Inverted for IntervalRoot {
    fn inverted(self) -> Self {
        IntervalRoot::from(Steps(7) - self.in_steps())
    }
}

/// # Example
///
/// ```
/// use solfege::interval::IntervalRoot;
/// use solfege::vertical::Steps;
///
/// assert_eq!(IntervalRoot::from(Steps(0)), IntervalRoot::Unison);
/// assert_eq!(IntervalRoot::from(Steps(4)), IntervalRoot::Fifth);
/// assert_eq!(IntervalRoot::from(Steps(-1)), IntervalRoot::Seventh);
/// assert_eq!(IntervalRoot::from(Steps(8)), IntervalRoot::Second);
/// ```
impl From<Steps> for IntervalRoot {
    fn from(steps: Steps) -> Self {
        match ((steps.0 % 7) + 7) % 7 {
            0 => Self::Unison,
            1 => Self::Second,
            2 => Self::Third,
            3 => Self::Fourth,
            4 => Self::Fifth,
            5 => Self::Sixth,
            6 => Self::Seventh,
            _ => panic!("Unreachable code"),
        }
    }
}

impl From<&PitchRootDyad> for IntervalRoot {
    fn from(dyad: &PitchRootDyad) -> Self {
        let steps = dyad.high.steps_from_c() - dyad.low.steps_from_c();

        IntervalRoot::from(steps)
    }
}

impl Display for IntervalRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.in_steps().0 + 1,)
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::IntervalRoot::{Fifth, Fourth, Second, Seventh, Sixth, Third, Unison};
    use crate::interval::Inverted;

    #[test]
    fn addition() {
        assert_eq!(Unison + Unison, Unison);
        assert_eq!(Unison + Third, Third);
        assert_eq!(Third + Fourth, Sixth);
    }

    #[test]
    fn wrapping_addition() {
        assert_eq!(Sixth + Fifth, Third);
    }

    #[test]
    fn subtraction() {
        assert_eq!(Unison - Unison, Unison);
        assert_eq!(Third - Unison, Third);
        assert_eq!(Third - Third, Unison);
        assert_eq!(Fifth - Third, Third);
    }

    #[test]
    fn wrapping_subtraction() {
        assert_eq!(Unison - Second, Seventh);
        assert_eq!(Third - Fifth, Sixth);
    }

    #[test]
    fn inversion() {
        assert_eq!(Unison.inverted(), Unison);
        assert_eq!(Third.inverted(), Sixth);
    }
}
