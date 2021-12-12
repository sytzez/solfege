use crate::pitch::{Accidental, Octave, Pitch, PitchClass, PitchRoot};
use crate::vertical::{Octaves, Semitones};

pub mod pitch;
pub mod vertical;
pub mod interval;
mod common;

fn main() {
    let c = PitchRoot::C;

    let c_sharp = PitchClass {
        root: c,
        accidental: Accidental {
            offset: Semitones(2)
        }
    };

    let c_sharp_4 = Pitch {
        class: c_sharp,
        octave: Octave {
            octaves_from_c0: Octaves(4)
        }
    };

    println!("{}", c.to_string());
    println!("{}", c_sharp.to_string());
    println!("{}", c_sharp_4.to_string());
}
