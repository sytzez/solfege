use crate::common::Scalar;
use crate::interval::IntervalRoot::Third;
use crate::interval::{minor, IntervalClass, IntervalRoot};
use crate::pitch::{
    natural_pitch_class_set, Accidental, Octave, Pitch, PitchClass, PitchClassSet, PitchRoot,
};
use crate::vertical::{Octaves, Semitones, SemitonesFromC, StepsFromC, TransposedBy};
use std::collections::BTreeSet;
use std::fmt::Display;

pub const FLAT: Accidental = Accidental {
    offset: Semitones(-1),
};
pub const NATURAL: Accidental = Accidental {
    offset: Semitones(0),
};
pub const SHARP: Accidental = Accidental {
    offset: Semitones(1),
};

pub trait PitchRootUtils {
    fn flat(&self) -> PitchClass;
    fn natural(&self) -> PitchClass;
    fn sharp(&self) -> PitchClass;
}

impl PitchRootUtils for PitchRoot {
    fn flat(&self) -> PitchClass {
        PitchClass {
            root: *self,
            accidental: FLAT,
        }
    }

    fn natural(&self) -> PitchClass {
        PitchClass {
            root: *self,
            accidental: NATURAL,
        }
    }

    fn sharp(&self) -> PitchClass {
        PitchClass {
            root: *self,
            accidental: SHARP,
        }
    }
}

pub trait PitchClassUtils {
    fn o(&self, octave_scalar: Scalar) -> Pitch;
    fn major(&self) -> PitchClassSet;
    fn minor(&self) -> PitchClassSet;
}

impl PitchClassUtils for PitchClass {
    fn o(&self, octave_scalar: Scalar) -> Pitch {
        Pitch {
            class: *self,
            octave: Octave {
                octaves_from_c0: Octaves(octave_scalar),
            },
        }
    }

    fn major(&self) -> PitchClassSet {
        let interval = IntervalClass {
            root: IntervalRoot::from(self.steps_from_c()),
            semitones: self.semitones_from_c(),
        };

        natural_pitch_class_set().transposed_by(&interval)
    }

    fn minor(&self) -> PitchClassSet {
        self.major().transposed_by(&minor(Third))
    }
}

impl PitchClassUtils for PitchRoot {
    fn o(&self, octave_scalar: Scalar) -> Pitch {
        self.natural().o(octave_scalar)
    }

    fn major(&self) -> PitchClassSet {
        self.natural().major()
    }

    fn minor(&self) -> PitchClassSet {
        self.natural().minor()
    }
}

pub fn print_set<T: Display>(set: &BTreeSet<T>) {
    let mut string = String::new();

    for class in set {
        if string.is_empty() {
            string = format!("{}", class);
        } else {
            string = format!("{}, {}", string, class);
        }
    }

    println!("{}", string);
}
