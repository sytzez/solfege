use crate::harmony::Dyad;
use crate::pitch::{Pitch, PitchClass, PitchRoot};

pub type PitchDyad = Dyad<Pitch>;
pub type PitchClassDyad = Dyad<PitchClass>;
pub type PitchRootDyad = Dyad<PitchRoot>;

impl From<&PitchDyad> for PitchClassDyad {
    fn from(pitch_dyad: &PitchDyad) -> Self {
        PitchClassDyad::from((pitch_dyad.low.class, pitch_dyad.high.class))
    }
}

impl From<&PitchDyad> for PitchRootDyad {
    fn from(pitch_dyad: &PitchDyad) -> Self {
        PitchRootDyad::from((pitch_dyad.low.class.root, pitch_dyad.high.class.root))
    }
}

impl From<&PitchClassDyad> for PitchRootDyad {
    fn from(pitch_class_dyad: &PitchClassDyad) -> Self {
        PitchRootDyad::from((pitch_class_dyad.low.root, pitch_class_dyad.high.root))
    }
}
