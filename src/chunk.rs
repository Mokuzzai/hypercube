
use crate::IndexableShape;
use crate::Shape;
use crate::SVector;
use crate::na;

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
