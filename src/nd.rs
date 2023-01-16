
use ndarray as nd;

pub use nd::Dimension;

pub type Array<T, const D: usize> = nd::Array<T, [usize; D]>;
