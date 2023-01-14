use crate::na;
use crate::Positions;
use crate::Shape;

pub trait Chunk<const D: usize> {
	type Item;
	type Shape: Shape<D>;

	const SHAPE: Self::Shape;

	fn shape(&self) -> Self::Shape {
		Self::SHAPE
	}

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: na::Vector<i32, D>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}

	fn get_mut(&mut self, position: na::Vector<i32, D>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
	}

	fn positions(&self) -> Positions<D> {
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
impl<C: Chunk<D>, P, const D: usize> Chunk<D> for WithPayload<C, P> {
	type Item = C::Item;
	type Shape = C::Shape;

	const SHAPE: Self::Shape = C::SHAPE;

	fn shape(&self) -> Self::Shape {
		self.chunk.shape()
	}

	fn index(&self, index: usize) -> Option<&Self::Item> {
		self.chunk.index(index)
	}

	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
		self.chunk.index_mut(index)
	}

	fn get(&self, position: na::Vector<i32, D>) -> Option<&Self::Item> {
		self.chunk.get(position)
	}

	fn get_mut(&mut self, position: na::Vector<i32, D>) -> Option<&mut Self::Item> {
		self.chunk.get_mut(position)
	}
}
