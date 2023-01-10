
use crate::IndexableShape;
use crate::Shape;
use crate::SVector;
use crate::na;

/// [`na::OVector`] used to index [`Chunk`]
pub type CVector<C> = SVector<<C as Chunk>::Shape>;

pub trait Chunk {
	type Item;
	type Shape: IndexableShape;

	fn shape(&self) -> &Self::Shape;

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: CVector<Self>) -> Option<&Self::Item>
	where
		na::DefaultAllocator: na::Allocator<<Self::Shape as Shape>::Coordinate, <Self::Shape as Shape>::Dimension>
	{
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}

	fn get_mut(&mut self, position: CVector<Self>) -> Option<&mut Self::Item>
	where
		na::DefaultAllocator: na::Allocator<<Self::Shape as Shape>::Coordinate, <Self::Shape as Shape>::Dimension>
	{
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
	}
}

/// Extends a [`Chunk`] with some payload
pub struct ChunkExt<C, P> {
	pub chunk: C,
	pub payload: P,
}

// ALERT: ALL METHODS MUST BE PASSED TROUGH EVEN IF THEY HAVE A DEFAULT IMPLEMENTATION
impl<C: Chunk, P> Chunk for ChunkExt<C, P> {
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

	fn get(&self, position: CVector<Self>) -> Option<&Self::Item>
	where
		na::DefaultAllocator: na::Allocator<<Self::Shape as Shape>::Coordinate, <Self::Shape as Shape>::Dimension>
	{
		self.chunk.get(position)
	}

	fn get_mut(&mut self, position: CVector<Self>) -> Option<&mut Self::Item>
	where
		na::DefaultAllocator: na::Allocator<<Self::Shape as Shape>::Coordinate, <Self::Shape as Shape>::Dimension>
	{
		self.chunk.get_mut(position)
	}
}
