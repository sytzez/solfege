use crate::interval::{
    GetIntervalQuality, IntervalClass, IntervalQuality, IntervalRoot, Inverted, IsPerfect,
};
use crate::pitch::PitchDyad;
use crate::vertical::{
    InOctaves, InSemitones, InSteps, Octaves, Semitones, SemitonesFromC0, Steps, StepsFromC0,
};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// Represents an [interval](https://en.wikipedia.org/wiki/Interval_(music)).
///
/// # Examples
///
/// ```
/// use solfege::interval::*;
/// use solfege::vertical::*;
///
/// let major_tenth = Interval {
///     class: IntervalClass {
///         root: IntervalRoot::Third,
///         semitones: Semitones(4),
///     },
///     octaves: Octaves(1),
/// };
///
/// assert_eq!(major_tenth.to_string(), "M10");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct Interval {
    pub octaves: Octaves,
    pub class: IntervalClass,
}

impl InSteps for Interval {
    fn in_steps(&self) -> Steps {
        self.class.in_steps() + self.octaves.in_steps()
    }
}

impl InSemitones for Interval {
    fn in_semitones(&self) -> Semitones {
        self.octaves.in_semitones() + self.class.in_semitones()
    }
}

impl IsPerfect for Interval {
    fn is_perfect(&self) -> bool {
        self.class.is_perfect()
    }
}

impl GetIntervalQuality for Interval {
    fn get_interval_quality(&self) -> IntervalQuality {
        self.class.get_interval_quality()
    }
}

impl Add for Interval {
    type Output = Interval;

    fn add(self, rhs: Self) -> Self::Output {
        Interval {
            octaves: (self.in_steps() + rhs.in_steps()).in_octaves(),
            class: self.class + rhs.class,
        }
    }
}

impl Sub for Interval {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            panic!("Can't subtract a larger Interval from a smaller Interval");
        }

        Interval {
            octaves: (self.in_steps() - rhs.in_steps()).in_octaves(),
            class: self.class - rhs.class,
        }
    }
}

impl Inverted for Interval {
    fn inverted(self) -> Self {
        panic!("Can't invert an Interval, invert an IntervalClass instead");
    }
}

/// Returns the interval of a pitch dyad
///
/// # Example
///
/// ```
/// use solfege::harmony::Dyad;
/// use solfege::interval::{augmented, Interval, IntervalClassUtils, major, minor};
/// use solfege::interval::IntervalRoot::{Second, Third};
/// use solfege::pitch::PitchRoot::{A, C, E, F};
/// use solfege::pitch::{PitchClassUtils, PitchRootUtils};
///
/// assert_eq!(Interval::from(&Dyad::from((C.o(4), E.o(4)))), major(Third).simple());
/// assert_eq!(Interval::from(&Dyad::from((A.o(4), C.o(5)))), minor(Third).simple());
/// assert_eq!(Interval::from(&Dyad::from((E.flat().o(4), F.sharp().o(4)))), augmented(Second).simple());
/// assert_eq!(Interval::from(&Dyad::from((C.o(4), E.o(5)))), major(Third).compound(1));
/// ```
impl From<&PitchDyad> for Interval {
    fn from(dyad: &PitchDyad) -> Self {
        let steps = dyad.high.steps_from_c0() - dyad.low.steps_from_c0();

        let semitones = dyad.high.semitones_from_c0() - dyad.low.semitones_from_c0();

        Interval {
            octaves: steps.in_octaves(),
            class: IntervalClass {
                root: IntervalRoot::from(Steps(steps.0 % 7)),
                semitones: Semitones(semitones.0 % 12),
            },
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.get_interval_quality(),
            self.in_steps().0 + 1,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::IntervalRoot::{Fifth, Fourth, Second, Sixth, Third, Unison};
    use crate::interval::{
        diminished, double_augmented, double_diminished, major, minor, perfect, Interval,
        IntervalClassUtils,
    };
    use crate::pitch::PitchRoot::{C, D, F};
    use crate::pitch::{PitchClassUtils, PitchDyad, PitchRootUtils};

    #[test]
    fn equality() {
        assert_eq!(major(Third).simple(), major(Third).simple(),);

        assert_ne!(major(Third).simple(), major(Third).compound(1),);

        assert_ne!(major(Third).simple(), minor(Third).simple(),);

        assert_ne!(major(Third).simple(), diminished(Fourth).simple(),)
    }

    #[test]
    fn order() {
        assert!(minor(Third).simple() < major(Third).simple());
        assert!(major(Third).simple() < diminished(Fourth).simple());
        assert!(major(Third).simple() < double_diminished(Fourth).simple());
        assert!(major(Third).simple() < major(Second).compound(1));
    }

    #[test]
    fn from_pitch_dyad() {
        assert_eq!(
            Interval::from(&PitchDyad::from((C.o(4), C.o(4)))),
            perfect(Unison).simple(),
        );

        assert_eq!(
            Interval::from(&PitchDyad::from((C.flat().o(4), C.sharp().o(4)))),
            double_augmented(Unison).simple(),
        );

        assert_eq!(
            Interval::from(&PitchDyad::from((D.o(4), F.o(5)))),
            minor(Third).compound(1),
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            perfect(Fifth).simple() + perfect(Fourth).simple(),
            perfect(Unison).compound(1),
        );

        assert_eq!(
            major(Sixth).simple() + perfect(Fifth).simple(),
            major(Third).compound(1),
        )
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            perfect(Unison).compound(1) - perfect(Fourth).simple(),
            perfect(Fifth).simple(),
        );
    }
}
