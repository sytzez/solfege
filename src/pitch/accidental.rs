use crate::common::Scalar;
use crate::vertical::{Semitones, TransposedBy};
use std::fmt::{Display, Formatter};

/// Represents an [accidental](https://en.wikipedia.org/wiki/Accidental_(music)).
///
/// ```
/// use solfege::pitch::*;
/// use solfege::vertical::*;
///
/// let sharp = Accidental { offset: Semitones(1) };
///
///  assert_eq!(sharp.to_string(), "♯");
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Accidental {
    pub offset: Semitones,
}

impl TransposedBy<Semitones> for Accidental {
    fn transposed_by(&self, delta: Semitones) -> Self {
        Accidental {
            offset: self.offset + delta,
        }
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self.offset.0 {
            -2 => String::from('𝄫'),
            -1 => String::from('♭'),
            0 => String::from('♮'),
            1 => String::from('♯'),
            2 => String::from('𝄪'),
            Scalar::MIN..=-3 => String::from('♭').repeat(-self.offset.0 as usize),
            3..=Scalar::MAX => String::from('♯').repeat(self.offset.0 as usize),
        };

        write!(f, "{}", string)
    }
}

impl TryFrom<&str> for Accidental {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut offset = 0;

        for char in value.chars() {
            offset += match char {
                '𝄫' => -2,
                '♭' => -1,
                '♮' => 0,
                '♯' => 1,
                '𝄪' => 2,
                _ => return Err("Could not parse Accidental"),
            }
        }

        Ok(Accidental {
            offset: offset.into(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::pitch::Accidental;
    use crate::vertical::{Semitones, TransposedBy};

    #[test]
    fn transposition() {
        assert_eq!(
            Accidental { offset: 0.into() }.transposed_by(Semitones(1)),
            Accidental { offset: 1.into() },
        );

        assert_eq!(
            Accidental { offset: 0.into() }.transposed_by(Semitones(-1)),
            Accidental {
                offset: (-1).into()
            },
        );
    }
}
