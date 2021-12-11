use crate::interval::{GetIntervalQuality, IntervalQuality, IsPerfect};
use crate::vertical::{AsSemitones, AsSteps, Semitones, Steps};

pub enum IntervalRoot {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl AsSteps for IntervalRoot {
    fn as_steps(&self) -> Steps {
        let value = match *self {
            Self::Unison => 0,
            Self::Second => 1,
            Self::Third => 2,
            Self::Fourth => 3,
            Self::Fifth => 4,
            Self::Sixth => 5,
            Self::Seventh => 6,
        };

        Steps::new(value)
    }
}

impl AsSemitones for IntervalRoot {
    fn as_semitones(&self) -> Semitones {
        let value = match *self {
            Self::Unison => 0,
            Self::Second => 2,
            Self::Third => 4,
            Self::Fourth => 5,
            Self::Fifth => 7,
            Self::Sixth => 9,
            Self::Seventh => 11,
        };

        Semitones::new(value)
    }
}

impl IsPerfect for IntervalRoot {
    fn is_perfect(&self) -> bool {
        match *self {
            Self::Unison => true,
            Self::Second => false,
            Self::Third => false,
            Self::Fourth => true,
            Self::Fifth => true,
            Self::Sixth => false,
            Self::Seventh => false,
        }
    }
}

impl GetIntervalQuality for IntervalRoot {
    fn get_interval_quality(&self) -> IntervalQuality {
        IntervalQuality::new(
            self.is_perfect(),
            Semitones::new(0),
        )
    }
}
