use crate::pitch::octave::Octave;
use crate::pitch::pitch_class::PitchClass;
use crate::vertical::semitones::{Semitones, SemitonesFromC, SemitonesFromC0};
use crate::vertical::steps::{Steps, StepsFromC, StepsFromC0};

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
