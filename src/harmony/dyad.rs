use std::hash::Hash;

/// A dyad is a pair of ordered values.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Dyad<T: Ord + Eq + PartialEq + Hash> {
    pub low: T,
    pub high: T,
}

impl<T: Ord + Eq + PartialEq + Hash> From<(T, T)> for Dyad<T> {
    fn from(tuple: (T, T)) -> Self {
        let (first, second) = tuple;

        if second > first {
            Self {
                low: first,
                high: second,
            }
        } else {
            Self {
                low: second,
                high: first,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::harmony::Dyad;

    #[test]
    fn order() {
        let dyad_1 = Dyad::from((1, 2));
        let dyad_2 = Dyad::from((2, 1));

        assert_eq!(dyad_1.low, 1);
        assert_eq!(dyad_1.high, 2);
        assert_eq!(dyad_2.low, 1);
        assert_eq!(dyad_2.high, 2);
    }

    #[test]
    fn equality() {
        assert_eq!(Dyad::from((1, 2)), Dyad::from((2, 1)));
    }
}
