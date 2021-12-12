use crate::interval::{GetIntervalQuality, IntervalQuality, IntervalRoot, IsPerfect};
use crate::vertical::{AsSemitones, AsSteps, Semitones, Steps};

/// Represents a simple interval with a quality.
///
/// # Examples
///
/// ```
/// use solfege::interval::*;
/// use solfege::vertical::*;
///
/// let major_third = IntervalClass {
///     root: IntervalRoot::Third,
///     semitones: Semitones(4),
/// };
///
/// assert_eq!(major_third.to_string(), "M3");
/// ```
#[derive(Copy, Clone)]
pub struct IntervalClass {
    pub root: IntervalRoot,
    pub semitones: Semitones,
}

impl AsSteps for IntervalClass {
    fn as_steps(&self) -> Steps {
        self.root.as_steps()
    }
}

impl AsSemitones for IntervalClass {
    fn as_semitones(&self) -> Semitones {
        self.semitones
    }
}

impl IsPerfect for IntervalClass {
    fn is_perfect(&self) -> bool {
        self.root.is_perfect() && self.semitones == self.root.as_semitones()
    }
}

impl GetIntervalQuality for IntervalClass {
    fn get_interval_quality(&self) -> IntervalQuality {
        IntervalQuality {
            is_perfectable: self.root.is_perfect(),
            offset: self.semitones - self.root.as_semitones(),
        }
    }
}

impl ToString for IntervalClass {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.get_interval_quality().to_string(),
            self.root.to_string(),
        )
    }
}
