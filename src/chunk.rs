use crate::na;
use crate::IndexableShape;
use crate::SVector;
use crate::Shape;

/// [`na::OVector`] used to index [`Chunk`]
pub type CVector<C> = SVector<<C as Chunk>::Shape>;

pub type CDim<C> = <<C as Chunk>::Shape as Shape>::Dim;

pub trait Chunk
where
	na::DefaultAllocator: na::Allocator<i32, CDim<Self>>,
{
	type Item;
	type Shape: IndexableShape;

	fn shape(&self) -> &Self::Shape;

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: CVector<Self>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}

	fn get_mut(&mut self, position: CVector<Self>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
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
impl<C: Chunk, P> Chunk for WithPayload<C, P>
where
	na::DefaultAllocator: na::Allocator<i32, CDim<C>>,
{
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

	fn get(&self, position: CVector<Self>) -> Option<&Self::Item> {
		self.chunk.get(position)
	}

	fn get_mut(&mut self, position: CVector<Self>) -> Option<&mut Self::Item> {
		self.chunk.get_mut(position)
	}
}
