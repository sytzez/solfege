use crate::common::Scalar;
use crate::vertical::{
    InSemitones, InSteps, Octaves, Semitones, SemitonesFromC0, Steps, StepsFromC0, TransposedBy,
};
use std::fmt::{Display, Formatter};
use std::ops::Sub;

/// Represents an [octave](https://en.wikipedia.org/wiki/Octave) range from C to B.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Octave {
    pub octaves_from_c0: Octaves,
}

impl StepsFromC0 for Octave {
    fn steps_from_c0(&self) -> Steps {
        self.octaves_from_c0.in_steps()
    }
}

impl SemitonesFromC0 for Octave {
    fn semitones_from_c0(&self) -> Semitones {
        self.octaves_from_c0.in_semitones()
    }
}

impl Sub for Octave {
    type Output = Octaves;

    fn sub(self, rhs: Self) -> Self::Output {
        self.octaves_from_c0 - rhs.octaves_from_c0
    }
}

impl TransposedBy<Octaves> for Octave {
    fn transposed_by(&self, delta: Octaves) -> Self {
        Octave {
            octaves_from_c0: self.octaves_from_c0 + delta,
        }
    }
}

impl Display for Octave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.octaves_from_c0.0)
    }
}

impl From<Scalar> for Octave {
    fn from(scalar: Scalar) -> Self {
        Octave {
            octaves_from_c0: scalar.into(),
        }
    }
}

impl TryFrom<&str> for Octave {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(value.parse::<Scalar>().map_err(|_| ())?.into())
    }
}

#[cfg(test)]
mod test {
    use crate::vertical::{Octaves, Semitones, SemitonesFromC0, Steps, StepsFromC0, TransposedBy};

    use super::Octave;

    #[test]
    fn steps_from_c0() {
        assert_eq!(Octave::from(0).steps_from_c0(), Steps(0));
        assert_eq!(Octave::from(4).steps_from_c0(), Steps(28));
    }

    #[test]
    fn semitones_from_c0() {
        assert_eq!(Octave::from(0).semitones_from_c0(), Semitones(0));
        assert_eq!(Octave::from(4).semitones_from_c0(), Semitones(48));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Octave::from(4) - Octave::from(1), Octaves(3));
        assert_eq!(Octave::from(4) - Octave::from(4), Octaves(0));
    }

    #[test]
    fn transposition() {
        assert_eq!(Octave::from(4).transposed_by(Octaves(1)), Octave::from(5));
        assert_eq!(Octave::from(4).transposed_by(Octaves(-1)), Octave::from(3));
    }

    #[test]
    fn display() {
        assert_eq!(Octave::from(4).to_string(), "4");
    }
}
