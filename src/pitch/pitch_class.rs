use crate::interval::IntervalClass;
use crate::pitch::{Accidental, PitchRoot};
use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC, TransposedBy};
use std::fmt::{Display, Formatter};

/// Represents a [pitch class](https://en.wikipedia.org/wiki/Pitch_class); a pitch root with an accidental.
///
/// # Examples
///
/// ```
/// use solfege::pitch::PitchRoot::*;
/// use solfege::pitch::PitchRootUtils;
///
/// assert_eq!(C.natural().to_string(), "C♮");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PitchClass {
    pub root: PitchRoot,
    pub accidental: Accidental,
}

impl StepsFromC for PitchClass {
    fn steps_from_c(&self) -> Steps {
        self.root.steps_from_c()
    }
}

impl SemitonesFromC for PitchClass {
    fn semitones_from_c(&self) -> Semitones {
        self.root.semitones_from_c() + self.accidental.offset
    }
}

impl TransposedBy<Steps> for PitchClass {
    fn transposed_by(&self, delta: Steps) -> Self {
        PitchClass {
            root: self.root.transposed_by(delta),
            accidental: self.accidental,
        }
    }
}

impl TransposedBy<Semitones> for PitchClass {
    fn transposed_by(&self, delta: Semitones) -> Self {
        PitchClass {
            root: self.root,
            accidental: self.accidental.transposed_by(delta),
        }
    }
}

impl TransposedBy<&IntervalClass> for PitchClass {
    fn transposed_by(&self, delta: &IntervalClass) -> Self {
        let new_root = self.root.transposed_by(delta.root);

        let mut semitones_between_roots = new_root.semitones_from_c() - self.root.semitones_from_c();

        // If the root has wrapped around, add another 12 semitones to get an accurate accidental shift
        if new_root < self.root {
            semitones_between_roots = semitones_between_roots + Semitones(12)
        }

        let new_accidental = Accidental {
            offset: self.accidental.offset + delta.semitones - semitones_between_roots,
        };

        PitchClass {
            root: new_root,
            accidental: new_accidental,
        }
    }
}

impl Display for PitchClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.root, self.accidental)
    }
}

impl TryFrom<&str> for PitchClass {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err("String is too short to be a Pitch Class");
        }

        let root = PitchRoot::try_from(&value[0..1])?;
        let accidental = Accidental::try_from(&value[1..])?;

        Ok(PitchClass { root, accidental })
    }
}

#[cfg(test)]
mod test {
    use crate::interval::major;
    use crate::interval::minor;
    use crate::interval::IntervalRoot::*;
    use crate::pitch::PitchRoot::*;
    use crate::pitch::PitchRootUtils;
    use crate::vertical::Semitones;
    use crate::vertical::SemitonesFromC;
    use crate::vertical::Steps;
    use crate::vertical::StepsFromC;
    use crate::vertical::TransposedBy;

    #[test]
    fn order() {
        assert!(C.flat() < C.natural());
        assert!(C.sharp() < D.natural());
        assert!(E.sharp() < F.flat());
    }

    #[test]
    fn steps_from_c() {
        assert_eq!(C.flat().steps_from_c(), Steps(0));
        assert_eq!(B.sharp().steps_from_c(), Steps(6));
    }

    #[test]
    fn semitones_from_c() {
        assert_eq!(C.natural().semitones_from_c(), Semitones(0));
        assert_eq!(F.sharp().semitones_from_c(), Semitones(6));
        assert_eq!(C.flat().semitones_from_c(), Semitones(-1));
        assert_eq!(B.sharp().semitones_from_c(), Semitones(12));
    }

    #[test]
    fn transposition_by_steps() {
        assert_eq!(D.flat().transposed_by(Steps(1)), E.flat());
        assert_eq!(A.sharp().transposed_by(Steps(2)), C.sharp());
    }

    #[test]
    fn transposition_by_semitones() {
        assert_eq!(D.flat().transposed_by(Semitones(1)), D.natural());
        assert_eq!(B.natural().transposed_by(Semitones(1)), B.sharp());
    }

    #[test]
    fn transposition_by_interval_class() {
        assert_eq!(D.flat().transposed_by(&major(Third)), F.natural());
        assert_eq!(B.flat().transposed_by(&minor(Third)), D.flat());
    }

    #[test]
    fn display() {
        assert_eq!(F.sharp().to_string(), "F♯");
    }
}
