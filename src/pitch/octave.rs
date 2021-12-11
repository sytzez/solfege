use crate::vertical::{AsSemitones, AsSteps, Octaves, Semitones, SemitonesFromC0, Steps, StepsFromC0};

pub struct Octave {
    octaves_from_c0: Octaves,
}

impl StepsFromC0 for Octave {
    fn steps_from_c0(&self) -> Steps {
        self.octaves_from_c0.as_steps()
    }
}

impl SemitonesFromC0 for Octave {
    fn semitones_from_c0(&self) -> Semitones {
        self.octaves_from_c0.as_semitones()
    }
}
