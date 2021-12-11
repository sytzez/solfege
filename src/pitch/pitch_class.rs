use crate::pitch::{Accidental, PitchRoot};
use crate::vertical::{Semitones, SemitonesFromC, StepsFromC};

pub struct PitchClass {
    root: PitchRoot,
    accidental: Accidental,
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
