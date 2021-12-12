use crate::common::Scalar;
use crate::vertical::Semitones;

#[derive(Copy, Clone)]
pub struct Accidental {
    pub offset: Semitones,
}

impl ToString for Accidental {
    fn to_string(&self) -> String {
        match self.offset.0 {
            -2 => String::from('𝄫'),
            -1 => String::from('♭'),
            0 => String::from('♮'),
            1 => String::from('♯'),
            2 => String::from('𝄪'),
            Scalar::MIN..=-3 => {
                String::from('♭').repeat(-self.offset.0 as usize)
            },
            3..=Scalar::MAX => {
                String::from('♯').repeat(self.offset.0 as usize)
            },
        }
    }
}
