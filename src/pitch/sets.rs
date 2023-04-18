use crate::common::Scalar;
use crate::harmony::Dyad;
use crate::interval::IntervalRoot::{Fifth, Fourth};
use crate::interval::{perfect, Interval, IntervalClass};
use crate::pitch::PitchRoot::{A, B, C, D, E, F, G};
use crate::pitch::{Octave, Pitch, PitchClass, PitchDyad, PitchRoot, PitchRootUtils};
use crate::vertical::{InSemitones, Octaves, Semitones, TransposedBy};
use std::collections::btree_set::BTreeSet;
use std::hash::Hash;

pub type PitchSet = BTreeSet<Pitch>;
pub type PitchClassSet = BTreeSet<PitchClass>;
pub type PitchRootSet = BTreeSet<PitchRoot>;

pub trait InRange<T: Hash + Ord> {
    fn in_range(&self, range: &Dyad<T>) -> BTreeSet<T>;
}

/// Creates a pitch set from pitch classes between a top and bottom pitch
///
/// # Example
///
/// ```
/// use solfege::pitch::{NATURAL, Octave, Pitch, PitchClass, PitchClassSet, InRange, PitchRootUtils, PitchClassUtils, PitchDyad};
/// use solfege::pitch::PitchRoot::{C, D, E, G};
/// use solfege::vertical::Octaves;
///
/// let pitch_class_set = PitchClassSet::from([C.natural(), E.natural(), G.natural()]);
///
/// let pitch_set = pitch_class_set.in_range(
///     &PitchDyad::from((
///         E.natural().o(3), // Starts at E3 natural
///         C.flat().o(5), // Should not include C5 natural
///     ))
/// );
///
/// assert!(pitch_set.contains(&E.natural().o(3)));
/// assert!(pitch_set.contains(&G.natural().o(3)));
/// assert!(pitch_set.contains(&C.natural().o(4)));
/// assert!(pitch_set.contains(&E.natural().o(4)));
/// assert!(pitch_set.contains(&G.natural().o(4)));
/// assert_eq!(pitch_set.len(), 5);
/// ```
impl InRange<Pitch> for PitchClassSet {
    fn in_range(&self, dyad: &PitchDyad) -> PitchSet {
        let mut pitches = PitchSet::new();

        for octave_scalar in dyad.low.octave.octaves_from_c0.0..=dyad.high.octave.octaves_from_c0.0
        {
            let octave = Octave {
                octaves_from_c0: Octaves(octave_scalar),
            };

            for &class in self {
                let pitch = Pitch { octave, class };

                if pitch >= dyad.low && pitch <= dyad.high {
                    pitches.insert(pitch);
                }
            }
        }

        pitches
    }
}

/// All the natural pitches, equal to C major or A minor.
pub fn natural_pitch_class_set() -> PitchClassSet {
    PitchClassSet::from([
        C.natural(),
        D.natural(),
        E.natural(),
        F.natural(),
        G.natural(),
        A.natural(),
        B.natural(),
    ])
}

/// All the natural pitches, plus an amount of flats and sharps based on the depth
///
/// # Example
///
/// ```
/// use solfege::pitch::PitchRoot::{A, B, C, D, E, F, G};
/// use solfege::pitch::{PitchRootUtils, tonal_pitch_class_set};
///
/// let pitch_class_set = tonal_pitch_class_set(2);
///
/// assert!(pitch_class_set.contains(&C.natural()));
/// assert!(pitch_class_set.contains(&D.natural()));
/// assert!(pitch_class_set.contains(&E.natural()));
/// assert!(pitch_class_set.contains(&F.natural()));
/// assert!(pitch_class_set.contains(&G.natural()));
/// assert!(pitch_class_set.contains(&A.natural()));
/// assert!(pitch_class_set.contains(&B.natural()));
/// assert!(pitch_class_set.contains(&B.flat()));
/// assert!(pitch_class_set.contains(&E.flat()));
/// assert!(pitch_class_set.contains(&F.sharp()));
/// assert!(pitch_class_set.contains(&C.sharp()));
/// assert_eq!(pitch_class_set.len(), 11);
/// ```
pub fn tonal_pitch_class_set(depth: usize) -> PitchClassSet {
    let mut set = natural_pitch_class_set();

    let mut dark = B.flat();
    let mut bright = F.sharp();

    for _ in 0..depth {
        set.insert(dark);
        set.insert(bright);

        dark = dark.transposed_by(&perfect(Fourth));
        bright = bright.transposed_by(&perfect(Fifth));
    }

    set
}

/// Transposes a whole pitch class set
///
/// # Example
///
/// ```
/// use solfege::interval::IntervalClass;
/// use solfege::interval::IntervalRoot::Second;
/// use solfege::pitch::{natural_pitch_class_set, PitchRootUtils};
/// use solfege::pitch::PitchRoot::{A, B, C, D, E, F, G};
/// use solfege::vertical::{Semitones, TransposedBy};
///
/// let set = natural_pitch_class_set();
///
/// let minor_second = IntervalClass { root: Second, semitones: Semitones(1) };
///
/// let transposed_set = natural_pitch_class_set().transposed_by(&minor_second);
///
/// // Should equal D flat major
/// assert!(transposed_set.contains(&D.flat()));
/// assert!(transposed_set.contains(&E.flat()));
/// assert!(transposed_set.contains(&F.natural()));
/// assert!(transposed_set.contains(&G.flat()));
/// assert!(transposed_set.contains(&A.flat()));
/// assert!(transposed_set.contains(&B.flat()));
/// assert!(transposed_set.contains(&C.natural()));
/// assert_eq!(transposed_set.len(), 7)
/// ```
impl TransposedBy<&IntervalClass> for PitchClassSet {
    fn transposed_by(&self, delta: &IntervalClass) -> Self {
        self.into_iter()
            .map(|class| class.transposed_by(delta))
            .collect::<PitchClassSet>()
    }
}

impl TransposedBy<&Interval> for PitchSet {
    fn transposed_by(&self, delta: &Interval) -> Self {
        self.into_iter()
            .map(|pitch| pitch.transposed_by(delta))
            .collect::<PitchSet>()
    }
}

/// Gives us the distance of all pitches in a set to a given pitch, in semitones
///
/// # Example
///
/// ```
/// use solfege::pitch::PitchRoot::{C, D, E, G};
/// use solfege::pitch::{distance_of_pitch_set_to_pitch, PitchClassUtils, PitchSet};
/// use solfege::vertical::Semitones;
///
/// let pitch_set = PitchSet::from([C.o(4), E.o(4), G.o(4)]);
///
/// let pitch = D.o(4);
///
/// assert_eq!(distance_of_pitch_set_to_pitch(&pitch_set, pitch), Semitones(9));
/// ```
pub fn distance_of_pitch_set_to_pitch(set: &PitchSet, pitch: Pitch) -> Semitones {
    if set.is_empty() {
        return Semitones(Scalar::MAX);
    }

    set.iter()
        .map(|pitch_in_set| {
            Interval::from(&PitchDyad::from((pitch, pitch_in_set.clone()))).in_semitones()
        })
        .fold(Semitones(0), |a, b| a + b)
}
