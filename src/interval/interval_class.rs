use crate::interval::{GetIntervalQuality, IntervalQuality, IntervalRoot, IsPerfect};
use crate::vertical::{AsSemitones, AsSteps, Semitones, Steps};

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
