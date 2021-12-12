use crate::interval::IntervalClass;
use crate::pitch::{Accidental, PitchRoot};
use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC, Transpose};

/// Represents a [pitch class](https://en.wikipedia.org/wiki/Pitch_class); a pitch root with an accidental.
///
/// # Examples
///
/// ```
/// use solfege::pitch::*;
/// use solfege::vertical::*;
///
/// let c_natural = PitchClass {
///     root: PitchRoot::C,
///     accidental: Accidental { offset: Semitones(0) }
/// };
///
/// assert_eq!(c_natural.to_string(), "C♮");
/// ```
#[derive(Copy, Clone)]
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

impl Transpose<Steps> for PitchClass {
    fn transpose(&self, delta: Steps) -> Self {
        PitchClass {
            root: self.root.transpose(delta),
            accidental: self.accidental,
        }
    }
}

impl Transpose<Semitones> for PitchClass {
    fn transpose(&self, delta: Semitones) -> Self {
        PitchClass {
            root: self.root,
            accidental: self.accidental.transpose(delta),
        }
    }
}

impl Transpose<&IntervalClass> for PitchClass {
    fn transpose(&self, delta: &IntervalClass) -> Self {
        PitchClass {
            root: self.root.transpose(&delta.root),
            accidental: self.accidental.transpose(delta.semitones), // TODO: wrong
        }
    }
}

impl ToString for PitchClass {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.root.to_string(),
            self.accidental.to_string(),
        )
    }
}
