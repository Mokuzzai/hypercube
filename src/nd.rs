
pub use ndarray::*;

pub type Shape<const D: usize> = [usize; D];

pub type Array<T, const D: usize> = ndarray::Array<T, Shape<D>>;
pub type ArrayView<'a, T, const D: usize> = ndarray::ArrayView<'a, T, Shape<D>>;
pub type ArrayViewMut<'a, T, const D: usize> = ndarray::ArrayViewMut<'a, T, Shape<D>>;
