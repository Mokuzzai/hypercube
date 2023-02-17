use super::*;

/// Extends a [`Chunk`] with some payload
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct WithPayload<C, P> {
	pub chunk: C,
	pub payload: P,
}

impl<C, P> WithPayload<C, P> {
	pub fn new(chunk: C, payload: P) -> Self {
		Self { chunk, payload }
	}
}

// ALERT: ALL METHODS MUST BE PASSED TROUGH EVEN IF THEY HAVE A DEFAULT IMPLEMENTATION
impl<C: Chunk<B>, P, const B: usize> Chunk<B> for WithPayload<C, P> {
	type Item = C::Item;
	type Shape = C::Shape;
	fn as_slice(&self) -> &[Self::Item] {
		self.chunk.as_slice()
	}
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self.chunk.as_mut_slice()
	}
	fn shape(&self) -> Cow<Self::Shape> {
		self.chunk.shape()
	}
	fn get(&self, position: math::Vector<i32, B>) -> Option<&Self::Item> {
		self.chunk.get(position)
	}
	fn get_mut(&mut self, position: math::Vector<i32, B>) -> Option<&mut Self::Item> {
		self.chunk.get_mut(position)
	}
	fn positions(&self) -> Positions<B> {
		self.chunk.positions()
	}
}
