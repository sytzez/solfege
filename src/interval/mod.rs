mod interval;
mod interval_class;
mod interval_quality;
mod interval_root;
mod sets;
mod utils;

pub use self::interval::*;
pub use self::interval_class::*;
pub use self::interval_quality::*;
pub use self::interval_root::*;
pub use self::sets::*;
pub use self::utils::*;

pub trait Inverted {
    fn inverted(self) -> Self;
}
