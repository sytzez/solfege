use crate::pitch::{Octave, PitchClass};
use crate::vertical::{Semitones, SemitonesFromC, SemitonesFromC0, Steps, StepsFromC, StepsFromC0};

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
        format!("{}{}", self.class.to_string(), self.octave.octaves_from_c0.0.to_string())
    }
}
