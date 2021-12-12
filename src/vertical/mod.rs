mod octaves;
mod semitones;
mod steps;

pub use self::octaves::*;
pub use self::semitones::*;
pub use self::steps::*;

pub trait Transpose<T> {
    /// Returns the current value transposed by `delta`.
    fn transpose(&self, delta: T) -> Self;
}
