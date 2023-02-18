use crate::math;
use crate::Chunk;
use crate::Cow;
use crate::Shape;

/// Implentation of a heap allocated [`Chunk`] with support for static and dynamic [`Shape`]s
#[derive(Debug)]
pub struct Boxed<T, S: Shape<B>, const B: usize> {
	shape: S,
	buffer: Box<[T]>,
}

impl<T, S: Shape<B>, const B: usize> Chunk<B> for Boxed<T, S, B> {
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

impl<T, S: Shape<B>, const B: usize> Boxed<T, S, B> {
	/// # Panics
	/// This function panics if `S.capacity() != buffer.len()`
	pub fn from_parts(shape: S, buffer: Box<[T]>) -> Self {
		assert_eq!(shape.capacity(), buffer.len());

		Self { shape, buffer }
	}
	pub fn from_shape_index(shape: S, mut f: impl FnMut(usize) -> T) -> Self {
		let capacity = shape.capacity();

		let mut buffer = Vec::with_capacity(capacity);

		for index in 0..capacity {
			buffer.push(f(index));
		}

		Self {
			buffer: buffer.into(),
			shape,
		}
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

impl<T, S: Shape<B>, const B: usize> Default for Boxed<T, S, B>
where
	T: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}
