use crate::interval::IntervalRoot;
use crate::vertical::{AsSteps, Semitones, SemitonesFromC, Steps, StepsFromC, Transpose};

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

impl From<Steps> for PitchRoot {
    /// Creates a root pitch from the number of steps from C,
    /// wrapping around if the steps are less than 0 or more than 6
    ///
    /// # Examples
    ///
    /// ```
    /// use solfege::pitch::PitchRoot;
    /// use solfege::vertical::Steps;
    ///
    /// assert_eq!(PitchRoot::from(Steps(2)).to_string(), "E");
    /// assert_eq!(PitchRoot::from(Steps(9)).to_string(), "E");
    /// assert_eq!(PitchRoot::from(Steps(-5)).to_string(), "E");
    /// ```
    fn from(steps_from_c: Steps) -> Self {
        match (steps_from_c.0 % 7 + 7) % 7 {
            0 => Self::C,
            1 => Self::D,
            2 => Self::E,
            3 => Self::F,
            4 => Self::G,
            5 => Self::A,
            6 => Self::B,
            _ => panic!("Unreachable code"),
        }
    }
}

impl Transpose<Steps> for PitchRoot {
    /// Transposes the root pitch by a number of steps,
    /// wrapping back to C if the result passes B
    ///
    /// # Examples
    ///
    /// ```
    /// use solfege::pitch::PitchRoot;
    /// use solfege::vertical::{Steps, Transpose};
    ///
    /// assert_eq!(PitchRoot::C.transpose(Steps(2)).to_string(), "E");
    /// assert_eq!(PitchRoot::B.transpose(Steps(2)).to_string(), "D");
    /// ```
    fn transpose(&self, delta: Steps) -> Self {
        Self::from(self.steps_from_c() + delta)
    }
}

impl Transpose<&IntervalRoot> for PitchRoot {
    /// Transposes the root pitch by an interval root,
    /// wrapping back to C if the result passes B
    ///
    /// # Examples
    ///
    /// ```
    /// use solfege::pitch::PitchRoot;
    /// use solfege::interval::IntervalRoot;
    /// use solfege::vertical::Transpose;
    ///
    /// assert_eq!(PitchRoot::C.transpose(&IntervalRoot::Third).to_string(), "E");
    /// assert_eq!(PitchRoot::B.transpose(&IntervalRoot::Third).to_string(), "D");
    /// ```
    fn transpose(&self, delta: &IntervalRoot) -> Self {
        self.transpose(delta.as_steps())
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
