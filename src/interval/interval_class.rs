use crate::interval::IntervalQuality::{Imperfect, Perfectable};
use crate::interval::IntervalRoot::Unison;
use crate::interval::{GetIntervalQuality, IntervalQuality, IntervalRoot, Inverted, IsPerfect};
use crate::pitch::{PitchClassDyad, PitchRootDyad};
use crate::vertical::{InOctaves, InSemitones, InSteps, Semitones, SemitonesFromC, Steps};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// Represents a simple interval with a quality.
///
/// # Examples
///
/// ```
/// use solfege::interval::*;
/// use solfege::vertical::*;
///
/// let major_third = IntervalClass {
///     root: IntervalRoot::Third,
///     semitones: Semitones(4),
/// };
///
/// assert_eq!(major_third.to_string(), "M3");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct IntervalClass {
    pub root: IntervalRoot,
    pub semitones: Semitones,
}

impl InSteps for IntervalClass {
    fn in_steps(&self) -> Steps {
        self.root.in_steps()
    }
}

impl InSemitones for IntervalClass {
    fn in_semitones(&self) -> Semitones {
        self.semitones
    }
}

impl IsPerfect for IntervalClass {
    fn is_perfect(&self) -> bool {
        self.root.is_perfect() && self.semitones == self.root.in_semitones()
    }
}

impl GetIntervalQuality for IntervalClass {
    fn get_interval_quality(&self) -> IntervalQuality {
        let offset = self.semitones - self.root.in_semitones();

        match self.root.is_perfect() {
            true => Perfectable(offset),
            false => Imperfect(offset),
        }
    }
}

impl Add for IntervalClass {
    type Output = IntervalClass;

    fn add(self, rhs: Self) -> Self::Output {
        let octaves = (self.in_steps() + rhs.in_steps()).in_octaves();

        IntervalClass {
            root: self.root + rhs.root,
            semitones: self.semitones + rhs.semitones - octaves.in_semitones(),
        }
    }
}

impl Sub for IntervalClass {
    type Output = IntervalClass;

    fn sub(self, rhs: Self) -> Self::Output {
        if self < rhs {
            return (rhs - self).inverted();
        }

        IntervalClass {
            root: self.root - rhs.root,
            semitones: self.semitones - rhs.semitones,
        }
    }
}

impl Inverted for IntervalClass {
    fn inverted(self) -> Self {
        if self.root == Unison {
            return IntervalClass {
                root: Unison,
                semitones: -self.semitones,
            };
        }

        IntervalClass {
            root: self.root.inverted(),
            semitones: Semitones(12) - self.semitones,
        }
    }
}

impl From<&PitchClassDyad> for IntervalClass {
    fn from(dyad: &PitchClassDyad) -> Self {
        let root = IntervalRoot::from(&PitchRootDyad::from(dyad));

        let semitones = dyad.high.semitones_from_c() - dyad.low.semitones_from_c();

        IntervalClass { root, semitones }
    }
}

impl Display for IntervalClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_interval_quality(), self.root)
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::IntervalRoot::{Fifth, Fourth, Second, Seventh, Sixth, Third, Unison};
    use crate::interval::{
        augmented, diminished, double_augmented, major, minor, perfect, Inverted,
    };

    #[test]
    fn addition() {
        assert_eq!(perfect(Fourth) + perfect(Unison), perfect(Fourth));

        assert_eq!(perfect(Fourth) + augmented(Unison), augmented(Fourth));

        assert_eq!(
            augmented(Fourth) + augmented(Unison),
            double_augmented(Fourth),
        );

        assert_eq!(perfect(Fourth) + perfect(Fourth), minor(Seventh));

        assert_eq!(minor(Third) + minor(Third), diminished(Fifth));

        assert_eq!(major(Third) + major(Third), augmented(Fifth));
    }

    #[test]
    fn wrapping_addition() {
        assert_eq!(major(Sixth) + perfect(Fifth), major(Third));

        assert_eq!(augmented(Fourth) + augmented(Fourth), augmented(Seventh));
    }

    #[test]
    fn subtraction() {
        assert_eq!(perfect(Fourth) - perfect(Unison), perfect(Fourth));

        assert_eq!(perfect(Fourth) - augmented(Unison), diminished(Fourth));

        assert_eq!(diminished(Fourth) - diminished(Fourth), perfect(Unison));

        assert_eq!(perfect(Fifth) - major(Third), minor(Third));

        assert_eq!(perfect(Fifth) - minor(Third), major(Third));
    }

    #[test]
    fn wrapping_subtraction() {
        assert_eq!(perfect(Unison) - minor(Third), major(Sixth));

        assert_eq!(major(Third) - perfect(Fifth), major(Sixth));
    }

    #[test]
    fn inversion() {
        assert_eq!(perfect(Unison).inverted(), perfect(Unison));

        assert_eq!(augmented(Unison).inverted(), diminished(Unison));

        assert_eq!(major(Second).inverted(), minor(Seventh));
    }
}
