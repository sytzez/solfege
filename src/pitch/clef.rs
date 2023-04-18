use crate::common::Scalar;
use crate::pitch::PitchRoot::{B, C, D};
use crate::pitch::{distance_of_pitch_set_to_pitch, Pitch, PitchClassUtils, PitchSet};
use crate::vertical::Semitones;
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// A [clef](https://en.wikipedia.org/wiki/Clef) is shown at the beginning of a stave to indicate which pitches are represented by the lines.
#[derive(Clone, Debug, Copy, Serialize, Eq, PartialEq, Hash)]
pub enum Clef {
    Treble,
    Alto,
    Bass,
}

pub type ClefSet = Vec<Clef>;

impl Clef {
    pub fn center(&self) -> Pitch {
        match *self {
            Self::Treble => B.o(4),
            Self::Alto => C.o(4),
            Self::Bass => D.o(3),
        }
    }
}

/// Gives you the most suitable clef for a set of pitches.
pub fn best_clef(clefs: &ClefSet, pitches: &PitchSet) -> Clef {
    if clefs.is_empty() {
        panic!("No clefs provided");
    }

    let mut shortest_distance = Semitones(Scalar::MAX);
    let mut best_clef = None;

    for clef in clefs {
        let distance = distance_of_pitch_set_to_pitch(pitches, clef.center());

        if distance < shortest_distance {
            shortest_distance = distance;
            best_clef = Some(*clef);
        }
    }

    return best_clef.unwrap();
}

impl Display for Clef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Clef::Treble => "treble",
            Clef::Alto => "alto",
            Clef::Bass => "bass",
        })
    }
}

impl TryFrom<&str> for Clef {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "treble" => Ok(Clef::Treble),
            "bass" => Ok(Clef::Bass),
            "alto" => Ok(Clef::Alto),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pitch::PitchClassUtils;
    use crate::pitch::PitchRoot::*;
    use crate::pitch::PitchSet;

    use super::best_clef;
    use super::Clef;
    use super::ClefSet;

    #[test]
    fn test_best_clef() {
        let clefs = ClefSet::from([Clef::Treble, Clef::Bass]);

        let pitches_treble = PitchSet::from([B.o(3), E.o(4), G.o(4)]);
        let pitches_bass = PitchSet::from([E.o(3), G.o(3), C.o(4)]);

        assert_eq!(best_clef(&clefs, &pitches_treble), Clef::Treble,);

        assert_eq!(best_clef(&clefs, &pitches_bass), Clef::Bass,);
    }
}
