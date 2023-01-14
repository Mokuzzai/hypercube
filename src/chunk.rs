use crate::na;
use crate::Positions;
use crate::Shape;

pub trait Chunk<const B: usize> {
	type Item;
	type Shape: Shape<B>;

	fn shape(&self) -> &Self::Shape;

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: na::Vector<i32, B>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}
	fn get_mut(&mut self, position: na::Vector<i32, B>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
	}
	fn replace(&mut self, position: na::Vector<i32, B>, with: Self::Item) -> Option<Self::Item> {
		Some(std::mem::replace(self.get_mut(position)?, with))
	}
	fn positions(&self) -> Positions<B> {
		self.shape().positions()
	}
}

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

	fn shape(&self) -> &Self::Shape {
		self.chunk.shape()
	}
	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.chunk.index(index)
	}
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.chunk.index_mut(index)
	}
	fn get(&self, position: na::Vector<i32, B>) -> Option<&Self::Item> {
		self.chunk.get(position)
	}
	fn get_mut(&mut self, position: na::Vector<i32, B>) -> Option<&mut Self::Item> {
		self.chunk.get_mut(position)
	}
	fn positions(&self) -> Positions<B> {
		self.chunk.positions()
	}
}
