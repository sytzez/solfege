use crate::common::Scalar;
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

impl ToString for IntervalQuality {
    fn to_string(&self) -> String {
        match self.is_perfectable {
            true => match self.offset.0 {
                -1 => String::from('d'),
                0 => String::from('P'),
                1 => String::from('A'),
                Scalar::MIN..=-2 => {
                    String::from('d').repeat(-self.offset.0 as usize)
                }
                2..=Scalar::MAX => {
                    String::from('A').repeat(self.offset.0 as usize)
                }
            },
            false => match self.offset.0 {
                -2 => String::from('d'),
                -1 => String::from('m'),
                0 => String::from('M'),
                1 => String::from('A'),
                Scalar::MIN..=-3 => {
                    String::from('d').repeat((1 - self.offset.0) as usize)
                }
                2..=Scalar::MAX => {
                    String::from('A').repeat(self.offset.0 as usize)
                }
            },
        }
    }
}
