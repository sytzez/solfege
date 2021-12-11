use crate::vertical::Semitones;

#[derive(Copy, Clone)]
pub struct IntervalQuality {
    pub is_perfectable: bool,
    pub offset: Semitones,
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
        self.is_perfectable && self.offset == Semitones(0)
    }
}
