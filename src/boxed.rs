
use crate::na;
use crate::DynamicMultiformShape;
use crate::Shape;
use crate::Chunk;
use crate::Cow;

#[derive(Debug)]
pub struct Boxed<T, S, const B: usize> {
	buffer: Box<[T]>,
	shape: S,
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
	pub fn new(shape: S) -> Self
	where
		T: Default,
	{
		let mut buffer = Vec::new();

		buffer.resize_with(shape.capacity(), T::default);

		Self { buffer: buffer.into(), shape }
	}
}
