use crate::interval::interval_quality::{GetIntervalQuality, IntervalQuality, IsPerfect};
use crate::interval::interval_root::IntervalRoot;
use crate::vertical::semitones::{AsSemitones, Semitones};
use crate::vertical::steps::{AsSteps, Steps};

pub struct IntervalClass {
    root: IntervalRoot,
    semitones: Semitones,
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
        IntervalQuality::new(
            is_perfectable: self.root.is_perfect(),
            self.semitones - self.root.as_semitones(),
        )
    }
}
