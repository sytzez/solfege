use crate::interval::Interval;
use crate::pitch::{Octave, PitchClass};
use crate::vertical::{Octaves, Semitones, SemitonesFromC, SemitonesFromC0, Steps, StepsFromC, StepsFromC0, TransposedBy};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

/// Represents a single [pitch](https://en.wikipedia.org/wiki/Pitch_(music)).
///
/// # Examples
///
/// ```
/// use solfege::pitch::PitchRoot::*;
/// use solfege::pitch::PitchClassUtils;
///
/// assert_eq!(C.o(4).to_string(), "C♮4");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Pitch {
    pub octave: Octave,
    pub class: PitchClass,
}

impl StepsFromC0 for Pitch {
    fn steps_from_c0(&self) -> Steps {
        self.octave.steps_from_c0() + self.class.steps_from_c()
    }
}

impl SemitonesFromC0 for Pitch {
    fn semitones_from_c0(&self) -> Semitones {
        self.octave.semitones_from_c0() + self.class.semitones_from_c()
    }
}

impl TransposedBy<&Interval> for Pitch {
    fn transposed_by(&self, delta: &Interval) -> Self {
        let new_class = self.class.transposed_by(&delta.class);

        let mut new_octave = self.octave.transposed_by(delta.octaves);

        // If the class has wrapped around, add another octave
        if new_class < self.class {
            new_octave = new_octave.transposed_by(Octaves(1))
        }

        Pitch {
            octave: new_octave,
            class: new_class,
        }
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.class, self.octave.octaves_from_c0.0)
    }
}

impl TryFrom<&str> for Pitch {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err("String is too short to be a Pitch");
        }

        let octave_index = value.find(char::is_numeric)
            .ok_or("String has no octave")?;

        let (class_slice, octave_slice) = value.split_at(octave_index);

        let class = PitchClass::try_from(class_slice)?;

        let octave = Octave::try_from(octave_slice).map_err(|_| "Could not parse octave")?;

        Ok(Pitch { class, octave })
    }
}

impl Serialize for Pitch {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(test)]
mod test {
    use crate::interval::{major, perfect};
    use crate::interval::IntervalClassUtils;
    use crate::interval::IntervalRoot::*;
    use crate::pitch::PitchRoot::*;
    use crate::pitch::PitchRootUtils;
    use crate::pitch::{Pitch, PitchClassUtils};
    use crate::vertical::Semitones;
    use crate::vertical::SemitonesFromC0;
    use crate::vertical::Steps;
    use crate::vertical::StepsFromC0;
    use crate::vertical::TransposedBy;

    #[test]
    fn steps_from_c0() {
        assert_eq!(E.flat().o(3).steps_from_c0(), Steps(23));
    }

    #[test]
    fn semitones_from_c0() {
        assert_eq!(E.flat().o(3).semitones_from_c0(), Semitones(39));
    }

    #[test]
    fn transposition() {
        assert_eq!(E.flat().o(3).transposed_by(&major(Third).simple()), G.o(3));
        assert_eq!(E.o(4).transposed_by(&perfect(Fifth).simple()), B.o(4));
        assert_eq!(G.o(4).transposed_by(&perfect(Fourth).simple()), C.o(5));
        assert_eq!(G.o(4).transposed_by(&perfect(Fourth).compound(1)), C.o(6));
    }

    #[test]
    fn display() {
        assert_eq!(C.o(4).to_string(), "C♮4");
    }

    #[test]
    fn from_str() {
        assert_eq!("C♮4".try_into(), Ok(C.o(4)))
    }
}
