use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC};

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
        match *self {
            Self::C => Steps(0),
            Self::D => Steps(1),
            Self::E => Steps(2),
            Self::F => Steps(3),
            Self::G => Steps(4),
            Self::A => Steps(5),
            Self::B => Steps(6),
        }
    }
}

impl SemitonesFromC for PitchRoot {
    fn semitones_from_c(&self) -> Semitones {
        match *self {
            Self::C => Semitones(0),
            Self::D => Semitones(2),
            Self::E => Semitones(4),
            Self::F => Semitones(5),
            Self::G => Semitones(7),
            Self::A => Semitones(9),
            Self::B => Semitones(11),
        }
    }
}
