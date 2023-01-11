mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;

use crate::SVector;
use crate::Shape;

use crate::Chunk;

use std::collections::BTreeMap;

/// How many dimensions does you [`World`] have?
pub struct WorldShape<const D: usize>;

impl<const D: usize> Shape for WorldShape<D> {
	type Dimension = na::Const<D>;
}

/// `N` dimensional space containing some [`Chunk`]s
pub struct World<S: Shape, C: Chunk>
where
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dimension>,
	na::DefaultAllocator: na::Allocator<i32, S::Dimension>,
{
	chunks: BTreeMap<OrderedVector<S>, C>,
}

impl<S: Shape, C: Chunk> World<S, C>
where
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dimension>,
	na::DefaultAllocator: na::Allocator<i32, S::Dimension>,
{
	pub fn new() -> Self {
		Self {
			chunks: BTreeMap::new(),
		}
	}
	pub fn chunk(&self, position: SVector<S>) -> Option<&C> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: SVector<S>) -> Option<&mut C> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn get_or_insert_with(&mut self, position: SVector<S>, chunk: impl FnMut() -> C) -> &mut C {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn block(&mut self, position: ()) -> Option<&C::Item> {
		// `position` is roughly Vector<max(world.shape.coordinates, world.chunk.shape.coordinates), max(world.shape.dimenison, world.chunk.shape.dimension)>
		todo!()
	}
	pub fn block_mut(&mut self, position: ()) -> Option<&mut C::Item> {
		todo!()
	}
	pub fn chunks(&self) -> ! {
		todo!()
	}
	pub fn chunks_mut(&mut self) -> ! {
		todo!()
	}
}

impl<S: Shape, C: Chunk> Default for World<S, C>
where
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dimension>,
	na::DefaultAllocator: na::Allocator<i32, S::Dimension>,
{
	fn default() -> Self {
		Self::new()
	}
}
