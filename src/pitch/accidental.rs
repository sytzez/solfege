use crate::common::Scalar;
use crate::vertical::Semitones;

#[derive(Copy, Clone)]
pub struct Accidental {
    pub offset: Semitones,
}

impl ToString for Accidental {
    fn to_string(&self) -> String {
        let Semitones(offset_value) = self.offset;

        match offset_value {
            -2 => String::from('𝄫'),
            -1 => String::from('♭'),
            0 => String::from('♮'),
            1 => String::from('♯'),
            2 => String::from('𝄪'),
            Scalar::MIN..=-3 => {
                String::from('♭')
                    .repeat(-offset_value as usize)
            },
            3..=Scalar::MAX => {
                String::from('♯')
                    .repeat(offset_value as usize)
            },
        }
    }
}
