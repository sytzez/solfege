use crate::pitch::{Octave, PitchClass};
use crate::vertical::{Semitones, SemitonesFromC, SemitonesFromC0, Steps, StepsFromC, StepsFromC0};

/// Represents a single [pitch](https://en.wikipedia.org/wiki/Pitch_(music)).
///
/// # Examples
///
/// ```
/// use solfege::pitch::*;
/// use solfege::vertical::*;
///
/// let middle_c = Pitch {
///     octave: Octave { octaves_from_c0: Octaves(4) },
///     class: PitchClass {
///         root: PitchRoot::C,
///         accidental: Accidental { offset: Semitones(0) },
///     },
/// };
///
/// assert_eq!(middle_c.to_string(), "C♮4");
/// ```
#[derive(Copy, Clone)]
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

impl ToString for Pitch {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.class.to_string(),
            self.octave.octaves_from_c0.0,
        )
    }
}
