use crate::math;
use crate::Chunk;
use crate::Cow;
use crate::Shape;

use std::array;

/// Implentation of a stack allocated [`Chunk`] with staticly known capacity
#[derive(Debug)]
pub struct Array<T, S, const B: usize, const N: usize> {
	shape: S,
	buffer: [T; N],
}

impl<T, S: Shape<B>, const B: usize, const N: usize> Chunk<B> for Array<T, S, B, N> {
	type Item = T;
	type Shape = S;

	fn shape(&self) -> Cow<Self::Shape> {
		Cow::Borrowed(&self.shape)
	}
	fn as_slice(&self) -> &[Self::Item] {
		&self.buffer
	}
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		&mut self.buffer
	}
}

impl<T, S: Shape<B>, const B: usize, const N: usize> Array<T, S, B, N> {
	/// # Panics
	/// This function panics if `S.capacity() != N`
	pub fn from_parts(shape: S, buffer: [T; N]) -> Self {
		assert_eq!(shape.capacity(), N);

		Self { shape, buffer }
	}
	pub fn from_shape_index(shape: S, mut f: impl FnMut(usize) -> T) -> Self {
		Self::from_parts(shape, array::from_fn(f))
	}
	pub fn from_shape_position(shape: S, mut f: impl FnMut(math::Vector<i32, B>) -> T) -> Self {
		let extents = shape.extents();

		Self::from_shape_index(shape, |index| {
			f(math::index_to_position(extents, index).unwrap_or_else(crate::lazy_unreachable!()))
		})
	}
	pub fn from_shape_default(shape: S) -> Self
	where
		T: Default,
	{
		Self::from_shape_index(shape, |_| T::default())
	}
	pub fn from_index(mut f: impl FnMut(usize) -> T) -> Self
	where
		S: Default,
	{
		Self::from_shape_index(S::default(), f)
	}
	pub fn from_position(mut f: impl FnMut(math::Vector<i32, B>) -> T) -> Self
	where
		S: Default,
	{
		Self::from_shape_position(S::default(), f)
	}
}

impl<T, S: Shape<B>, const B: usize, const N: usize> Default for Array<T, S, B, N>
where
	T: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}
