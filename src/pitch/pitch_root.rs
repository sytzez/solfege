use crate::interval::IntervalRoot;
use crate::vertical::{InSteps, Semitones, SemitonesFromC, Steps, StepsFromC, TransposedBy};
use std::fmt::{Display, Formatter};

/// Represents one of the seven roots a pitch can have: C, D, E, F, G, A or B.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PitchRoot {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl StepsFromC for PitchRoot {
    fn steps_from_c(&self) -> Steps {
        let value = match *self {
            Self::C => 0,
            Self::D => 1,
            Self::E => 2,
            Self::F => 3,
            Self::G => 4,
            Self::A => 5,
            Self::B => 6,
        };

        Steps(value)
    }
}

impl SemitonesFromC for PitchRoot {
    fn semitones_from_c(&self) -> Semitones {
        let value = match *self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        };

        Semitones(value)
    }
}

impl From<Steps> for PitchRoot {
    /// Creates a root pitch from the number of steps from C,
    /// wrapping around if the steps are less than 0 or more than 6
    fn from(steps_from_c: Steps) -> Self {
        match (steps_from_c.0 % 7 + 7) % 7 {
            0 => Self::C,
            1 => Self::D,
            2 => Self::E,
            3 => Self::F,
            4 => Self::G,
            5 => Self::A,
            6 => Self::B,
            _ => panic!("Unreachable code"),
        }
    }
}

impl TransposedBy<Steps> for PitchRoot {
    /// Transposes the root pitch by a number of steps,
    /// wrapping back to C if the result passes B
    fn transposed_by(&self, delta: Steps) -> Self {
        Self::from(self.steps_from_c() + delta)
    }
}

impl TransposedBy<IntervalRoot> for PitchRoot {
    /// Transposes the root pitch by an interval root,
    /// wrapping back to C if the result passes B
    fn transposed_by(&self, delta: IntervalRoot) -> Self {
        self.transposed_by(delta.in_steps())
    }
}

impl Display for PitchRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match *self {
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::A => 'A',
            Self::B => 'B',
        };

        write!(f, "{}", char)
    }
}

impl TryFrom<&str> for PitchRoot {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err("Could not recognise pitch root"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::IntervalRoot::*;
    use super::PitchRoot::*;
    use crate::{
        pitch::PitchRoot,
        vertical::{Semitones, SemitonesFromC, Steps, StepsFromC, TransposedBy},
    };

    #[test]
    fn order() {
        assert!(C < D);
        assert!(C < B);
    }

    #[test]
    fn steps_from_c() {
        assert_eq!(C.steps_from_c(), Steps(0));
        assert_eq!(B.steps_from_c(), Steps(6));
    }

    #[test]
    fn semitones_from_c() {
        assert_eq!(C.semitones_from_c(), Semitones(0));
        assert_eq!(B.semitones_from_c(), Semitones(11));
    }

    #[test]
    fn from_steps() {
        assert_eq!(PitchRoot::from(Steps(2)), E);
        assert_eq!(PitchRoot::from(Steps(9)), E);
        assert_eq!(PitchRoot::from(Steps(-5)), E);
    }

    #[test]
    fn transposed_by_steps() {
        assert_eq!(C.transposed_by(Steps(2)), E);
        assert_eq!(B.transposed_by(Steps(2)), D);
    }

    #[test]
    fn transposed_by_interval_root() {
        assert_eq!(C.transposed_by(Third), E);
        assert_eq!(B.transposed_by(Third), D);
    }

    #[test]
    fn display() {
        assert_eq!(C.to_string(), "C");
    }
}
