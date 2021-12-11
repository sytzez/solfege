use crate::pitch::{Octave, PitchClass};
use crate::vertical::{SemitonesFromC, SemitonesFromC0, StepsFromC, StepsFromC0};

pub struct Pitch {
    octave: Octave,
    class: PitchClass,
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
