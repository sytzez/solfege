use crate::interval::{GetIntervalQuality, IntervalClass, IntervalQuality, IsPerfect};
use crate::pitch::Pitch;
use crate::vertical::{AsSemitones, AsSteps, Octaves, Semitones, Steps};

pub struct Interval {
    class: IntervalClass,
    octaves: Octaves,
}

impl Interval {
    pub fn between_pitches(from: Pitch, to: Pitch) -> Self {
        // TODO
    }
}

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
