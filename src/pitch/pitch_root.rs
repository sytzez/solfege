use crate::vertical::{Semitones, SemitonesFromC, Steps, StepsFromC};

/// Represents one of the seven roots a pitch can have: C, D, E, F, G, A or B.
///
/// # Examples
///
/// ```
/// use solfege::pitch::PitchRoot;
///
/// let c = PitchRoot::C;
///
/// assert_eq!("C", c.to_string())
/// ```
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

impl ToString for PitchRoot {
    fn to_string(&self) -> String {
        let char = match *self {
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            Self::A => 'A',
            Self::B => 'B',
        };

        String::from(char)
    }
}
