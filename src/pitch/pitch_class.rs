use crate::pitch::accidental::Accidental;
use crate::pitch::pitch_root::PitchRoot;
use crate::vertical::semitones::{Semitones, SemitonesFromC};
use crate::vertical::steps::{Steps, StepsFromC};

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
