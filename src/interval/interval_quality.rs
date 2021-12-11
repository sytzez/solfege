use crate::vertical::Semitones;

pub struct IntervalQuality {
    is_perfectable: bool,
    offset: Semitones,
}

pub trait GetIntervalQuality {
    fn get_interval_quality(&self) -> IntervalQuality;
}

pub trait IsPerfect {
    fn is_perfect(&self) -> bool;
}

impl IntervalQuality {
    pub fn new(is_perfectable: bool, offset: Semitones) -> Self {
        Self { is_perfectable, offset }
    }
}

impl IsPerfect for IntervalQuality {
    fn is_perfect(&self) -> bool {
        self.is_perfectable && self.offset == Offset(0)
    }
}
