mod octaves;
mod semitones;
mod steps;

pub use self::octaves::*;
pub use self::semitones::*;
pub use self::steps::*;

pub trait TransposedBy<T> {
    /// Returns the current value transposed by `delta`.
    fn transposed_by(&self, delta: T) -> Self;
}
