use crate::pitch::{Accidental, PitchRoot};
use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC};

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

impl ToString for PitchClass {
    fn to_string(&self) -> String {
        format!("{}{}", self.root.to_string(), self.accidental.to_string())
    }
}
