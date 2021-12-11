use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC};

#[derive(Copy, Clone)]
pub enum PitchRoot {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl StepsFromC for PitchRoot {
    fn steps_from_c(&self) -> Steps {
        let value = match *self {
            Self::C => 0,
            Self::D => 1,
            Self::E => 2,
            Self::F => 3,
            Self::G => 4,
            Self::A => 5,
            Self::B => 6,
        };

        Steps(value)
    }
}

impl SemitonesFromC for PitchRoot {
    fn semitones_from_c(&self) -> Semitones {
        let value = match *self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        };

        Semitones(value)
    }
}
