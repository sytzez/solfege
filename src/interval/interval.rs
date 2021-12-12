use crate::interval::{GetIntervalQuality, IntervalClass, IntervalQuality, IsPerfect};
use crate::vertical::{AsSemitones, AsSteps, Octaves, Semitones, Steps};

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
#[derive(Copy, Clone)]
pub struct Interval {
    pub class: IntervalClass,
    pub octaves: Octaves,
}

// from_steps_and_semitones
// between_pitches

impl AsSteps for Interval {
    fn as_steps(&self) -> Steps {
        self.class.as_steps() + self.octaves.as_steps()
    }
}

impl AsSemitones for Interval {
    fn as_semitones(&self) -> Semitones {
        self.octaves.as_semitones() + self.class.as_semitones()
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

impl ToString for Interval {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.get_interval_quality().to_string(),
            self.as_steps().0 + 1,
        )
    }
}
