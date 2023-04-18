use crate::common::Scalar;
use crate::interval::{Interval, IntervalClass, IntervalRoot, IsPerfect};
use crate::vertical::{InSemitones, Octaves, Semitones};

/// Creates a perfect interval
pub fn perfect(root: IntervalRoot) -> IntervalClass {
    assert!(root.is_perfect());

    IntervalClass {
        root,
        semitones: root.in_semitones(),
    }
}

/// Creates a minor interval
pub fn minor(root: IntervalRoot) -> IntervalClass {
    assert!(!root.is_perfect());

    IntervalClass {
        root,
        semitones: root.in_semitones() - Semitones(1),
    }
}

/// Creates a major interval
pub fn major(root: IntervalRoot) -> IntervalClass {
    assert!(!root.is_perfect());

    IntervalClass {
        root,
        semitones: root.in_semitones(),
    }
}

/// Creates a diminished interval
pub fn diminished(root: IntervalRoot) -> IntervalClass {
    let offset = if root.is_perfect() {
        Semitones(1)
    } else {
        Semitones(2)
    };

    IntervalClass {
        root,
        semitones: root.in_semitones() - offset,
    }
}

/// Creates a double diminished interval
pub fn double_diminished(root: IntervalRoot) -> IntervalClass {
    let offset = if root.is_perfect() {
        Semitones(2)
    } else {
        Semitones(3)
    };

    IntervalClass {
        root,
        semitones: root.in_semitones() - offset,
    }
}

/// Creates an augmented interval
pub fn augmented(root: IntervalRoot) -> IntervalClass {
    IntervalClass {
        root,
        semitones: root.in_semitones() + Semitones(1),
    }
}

/// Creates a double augmented interval
pub fn double_augmented(root: IntervalRoot) -> IntervalClass {
    IntervalClass {
        root,
        semitones: root.in_semitones() + Semitones(2),
    }
}

pub trait IntervalClassUtils {
    /// Creates a 'simple' version of the interval, e.g. smaller than an octave
    fn simple(&self) -> Interval;

    /// Compounds an interval by a number of octaves
    fn compound(&self, num_of_octaves: Scalar) -> Interval;
}

impl IntervalClassUtils for IntervalClass {
    fn simple(&self) -> Interval {
        self.compound(0)
    }

    fn compound(&self, num_of_octaves: Scalar) -> Interval {
        Interval {
            octaves: Octaves(num_of_octaves),
            class: *self,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interval::IntervalRoot::{Fifth, Fourth, Second, Seventh, Sixth, Third, Unison};
    use crate::interval::{
        augmented, diminished, double_augmented, double_diminished, major, minor, perfect,
        Interval, IntervalClass, IntervalClassUtils,
    };
    use std::panic;

    #[test]
    fn test_perfect() {
        assert_eq!(
            perfect(Unison),
            IntervalClass {
                root: Unison,
                semitones: 0.into(),
            }
        );

        assert_eq!(
            perfect(Fourth),
            IntervalClass {
                root: Fourth,
                semitones: 5.into(),
            }
        );

        assert_eq!(
            perfect(Fifth),
            IntervalClass {
                root: Fifth,
                semitones: 7.into(),
            }
        );
    }

    #[test]
    fn test_perfect_panics() {
        assert!(panic::catch_unwind(|| perfect(Second)).is_err());
        assert!(panic::catch_unwind(|| perfect(Third)).is_err());
        assert!(panic::catch_unwind(|| perfect(Sixth)).is_err());
        assert!(panic::catch_unwind(|| perfect(Seventh)).is_err());
    }

    #[test]
    fn test_minor() {
        assert_eq!(
            minor(Second),
            IntervalClass {
                root: Second,
                semitones: 1.into(),
            }
        );

        assert_eq!(
            minor(Third),
            IntervalClass {
                root: Third,
                semitones: 3.into(),
            }
        );

        assert_eq!(
            minor(Sixth),
            IntervalClass {
                root: Sixth,
                semitones: 8.into(),
            }
        );

        assert_eq!(
            minor(Seventh),
            IntervalClass {
                root: Seventh,
                semitones: 10.into(),
            }
        );
    }

    #[test]
    fn test_minor_panics() {
        assert!(panic::catch_unwind(|| minor(Unison)).is_err());
        assert!(panic::catch_unwind(|| minor(Fourth)).is_err());
        assert!(panic::catch_unwind(|| minor(Fifth)).is_err());
    }

    #[test]
    fn test_major() {
        assert_eq!(
            major(Second),
            IntervalClass {
                root: Second,
                semitones: 2.into(),
            }
        );

        assert_eq!(
            major(Third),
            IntervalClass {
                root: Third,
                semitones: 4.into(),
            }
        );

        assert_eq!(
            major(Sixth),
            IntervalClass {
                root: Sixth,
                semitones: 9.into(),
            }
        );

        assert_eq!(
            major(Seventh),
            IntervalClass {
                root: Seventh,
                semitones: 11.into(),
            }
        );
    }

    #[test]
    fn test_major_panics() {
        assert!(panic::catch_unwind(|| major(Unison)).is_err());
        assert!(panic::catch_unwind(|| major(Fourth)).is_err());
        assert!(panic::catch_unwind(|| major(Fifth)).is_err());
    }

    #[test]
    fn test_diminished() {
        assert_eq!(
            diminished(Third),
            IntervalClass {
                root: Third,
                semitones: 2.into(),
            }
        );

        assert_eq!(
            diminished(Fourth),
            IntervalClass {
                root: Fourth,
                semitones: 4.into(),
            }
        );
    }

    #[test]
    fn test_double_diminished() {
        assert_eq!(
            double_diminished(Third),
            IntervalClass {
                root: Third,
                semitones: 1.into(),
            }
        );

        assert_eq!(
            double_diminished(Fourth),
            IntervalClass {
                root: Fourth,
                semitones: 3.into(),
            }
        );
    }

    #[test]
    fn test_augmented() {
        assert_eq!(
            augmented(Second),
            IntervalClass {
                root: Second,
                semitones: 3.into(),
            }
        );

        assert_eq!(
            augmented(Fourth),
            IntervalClass {
                root: Fourth,
                semitones: 6.into(),
            }
        );
    }

    #[test]
    fn test_double_augmented() {
        assert_eq!(
            double_augmented(Second),
            IntervalClass {
                root: Second,
                semitones: 4.into(),
            }
        );

        assert_eq!(
            double_augmented(Fourth),
            IntervalClass {
                root: Fourth,
                semitones: 7.into(),
            }
        );
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            perfect(Unison).simple(),
            Interval {
                class: IntervalClass {
                    root: Unison,
                    semitones: 0.into(),
                },
                octaves: 0.into(),
            }
        );

        assert_eq!(
            major(Seventh).simple(),
            Interval {
                class: IntervalClass {
                    root: Seventh,
                    semitones: 11.into(),
                },
                octaves: 0.into(),
            }
        )
    }

    #[test]
    fn test_compound() {
        assert_eq!(
            perfect(Unison).compound(1),
            Interval {
                class: IntervalClass {
                    root: Unison,
                    semitones: 0.into(),
                },
                octaves: 1.into(),
            }
        );

        assert_eq!(
            major(Seventh).compound(2),
            Interval {
                class: IntervalClass {
                    root: Seventh,
                    semitones: 11.into(),
                },
                octaves: 2.into(),
            }
        )
    }
}
