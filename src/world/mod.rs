mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;
use crate::Chunk;
use crate::Shape;

use std::collections::BTreeMap;

/// `N` dimensional space containing some [`Chunk`]s
///
/// * `C`: `Chunk`
/// * `E`: dimensions in the world
/// * `V`: dimensions in a chunk
pub struct World<C: Chunk<V>, const E: usize, const V: usize> {
	chunks: BTreeMap<OrderedVector<E>, C>,
}

impl<C: Chunk<V>, const E: usize, const V: usize> World<C, E, V> {
	pub fn new() -> Self {
		Self {
			chunks: BTreeMap::new(),
		}
	}
	pub fn chunk(&self, position: na::Vector<i32, E>) -> Option<&C> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: na::Vector<i32, E>) -> Option<&mut C> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn get_or_insert_with(&mut self, position: na::Vector<i32, E>, chunk: impl FnMut() -> C) -> &mut C {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&na::Vector<i32, E>, &C)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&na::Vector<i32, E>, &mut C)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &na::Vector<i32, E>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &C> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut C> {
		self.chunks.values_mut()
	}
}

// impl<S: Shape, C: Chunk> World<S, C>
// where
// 	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
// 	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
// 	na::DefaultAllocator: na::Allocator<i32, WDim<S, C>>,
// 	SDim<S>: na::DimMax<CDim<C>>,
// {
// 	pub fn block(&mut self, position: WVector<S, C>) -> Option<&C::Item> {
// 		// `position` is roughly Vector<max(world.shape.coordinates, world.chunk.shape.coordinates), max(world.shape.dimenison, world.chunk.shape.dimension)>
// 		todo!()
// 	}
// 	pub fn block_mut(&mut self, position: WVector<S, C>) -> Option<&mut C::Item> {
// 		todo!()
// 	}
// }

impl<C: Chunk<V>, const E: usize, const V: usize> Default for World<C, E, V> {
	fn default() -> Self {
		Self::new()
	}
}

const _: () = {
	use std::fmt::*;

	impl<C: Chunk<V>, const E: usize, const V: usize> Debug for World<C, E, V>
	where
		C: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Wolrd")
				.field("chunks", &self.chunks)
				.finish()
		}
	}
};
