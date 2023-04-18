use crate::common::Scalar;
use crate::vertical::Semitones;
use std::fmt::{Display, Formatter};

/// Represents the [quality](https://en.wikipedia.org/wiki/Interval_(music)#Quality) of an interval, such as perfect, minor and major.
///
/// # Examples
///
/// ```
/// use solfege::interval::{GetIntervalQuality, Interval};
/// use solfege::interval::IntervalQuality::{Imperfect};
/// use solfege::pitch::{PitchClassUtils, PitchDyad};
/// use solfege::pitch::PitchRoot::{D, F};
/// use solfege::vertical::Semitones;
///
/// // The interval between D and F is a minor third, which is imperfect
/// assert_eq!(
///     Interval::from(&PitchDyad::from((D.o(4), F.o(4)))).get_interval_quality(),
///     Imperfect(Semitones(-1)),
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum IntervalQuality {
    Perfectable(Semitones),
    Imperfect(Semitones),
}

pub trait GetIntervalQuality {
    /// Returns the quality of the interval.
    fn get_interval_quality(&self) -> IntervalQuality;
}

pub trait IsPerfect {
    /// Returns whether the interval is perfect.
    fn is_perfect(&self) -> bool;
}

impl IsPerfect for IntervalQuality {
    fn is_perfect(&self) -> bool {
        *self == Self::Perfectable(Semitones(0))
    }
}

impl Display for IntervalQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Perfectable(offset) => match offset.0 {
                -1 => String::from('d'),
                0 => String::from('P'),
                1 => String::from('A'),
                Scalar::MIN..=-2 => String::from('d').repeat(-offset.0 as usize),
                2..=Scalar::MAX => String::from('A').repeat(offset.0 as usize),
            },
            Self::Imperfect(offset) => match offset.0 {
                -2 => String::from('d'),
                -1 => String::from('m'),
                0 => String::from('M'),
                1 => String::from('A'),
                Scalar::MIN..=-3 => String::from('d').repeat((-offset.0 - 1) as usize),
                2..=Scalar::MAX => String::from('A').repeat(offset.0 as usize),
            },
        };

        write!(f, "{}", string)
    }
}

#[cfg(test)]
mod test {
    use crate::interval::IntervalQuality::{Imperfect, Perfectable};
    use crate::vertical::Semitones;

    #[test]
    fn display() {
        assert_eq!(Imperfect(Semitones(-3)).to_string(), "dd");
        assert_eq!(Imperfect(Semitones(-2)).to_string(), "d");
        assert_eq!(Imperfect(Semitones(-1)).to_string(), "m");
        assert_eq!(Imperfect(Semitones(0)).to_string(), "M");
        assert_eq!(Imperfect(Semitones(1)).to_string(), "A");
        assert_eq!(Imperfect(Semitones(2)).to_string(), "AA");

        assert_eq!(Perfectable(Semitones(-2)).to_string(), "dd");
        assert_eq!(Perfectable(Semitones(-1)).to_string(), "d");
        assert_eq!(Perfectable(Semitones(0)).to_string(), "P");
        assert_eq!(Perfectable(Semitones(1)).to_string(), "A");
        assert_eq!(Perfectable(Semitones(2)).to_string(), "AA");
    }
}
