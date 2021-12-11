use crate::interval::interval_quality::{GetIntervalQuality, IntervalQuality, IsPerfect};
use crate::vertical::semitones::{AsSemitones, Semitones};
use crate::vertical::steps::{AsSteps, Steps};

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
        match *self {
            Self::Unison => Steps(0),
            Self::Second => Steps(1),
            Self::Third => Steps(2),
            Self::Fourth => Steps(3),
            Self::Fifth => Steps(4),
            Self::Sixth => Steps(5),
            Self::Seventh => Steps(6),
        }
    }
}

impl AsSemitones for IntervalRoot {
    fn as_semitones(&self) -> Semitones {
        match *self {
            Self::Unison => Semitones(0),
            Self::Second => Semitones(2),
            Self::Third => Semitones(4),
            Self::Fourth => Semitones(5),
            Self::Fifth => Semitones(7),
            Self::Sixth => Semitones(9),
            Self::Seventh => Semitones(11),
        }
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
            Offset(0),
        )
    }
}
