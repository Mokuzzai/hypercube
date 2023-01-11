mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;
use crate::CDim;
use crate::Chunk;
use crate::SDim;
use crate::SVector;
use crate::Shape;

use std::collections::BTreeMap;

/// How many dimensions does you [`World`] have?
#[derive(Debug, Default, Copy, Clone)]
pub struct WorldShape<const D: usize>;

pub type WDim<S, C> = na::DimMaximum<<S as Shape>::Dim, <<C as Chunk>::Shape as Shape>::Dim>;
pub type WVector<S, C> = na::OVector<i32, WDim<S, C>>;

impl<const D: usize> Shape for WorldShape<D> {
	type Dim = na::Const<D>;
}

/// `N` dimensional space containing some [`Chunk`]s
pub struct World<S: Shape, C: Chunk>
where
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	chunks: BTreeMap<OrderedVector<S>, C>,
}

impl<S: Shape, C: Chunk> World<S, C>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
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
	pub fn iter(&self) -> impl Iterator<Item = (&SVector<S>, &C)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&SVector<S>, &mut C)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &SVector<S>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &C> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut C> {
		self.chunks.values_mut()
	}
}

impl<S: Shape, C: Chunk> World<S, C>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
	na::DefaultAllocator: na::Allocator<i32, WDim<S, C>>,
	SDim<S>: na::DimMax<CDim<C>>,
{
	pub fn block(&mut self, position: WVector<S, C>) -> Option<&C::Item> {
		// `position` is roughly Vector<max(world.shape.coordinates, world.chunk.shape.coordinates), max(world.shape.dimenison, world.chunk.shape.dimension)>
		todo!()
	}
	pub fn block_mut(&mut self, position: WVector<S, C>) -> Option<&mut C::Item> {
		todo!()
	}
}

impl<S: Shape, C: Chunk> Default for World<S, C>
where
	na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	fn default() -> Self {
		Self::new()
	}
}

const _: () = {
	use std::fmt::*;

	impl<S: Shape, C: Chunk> Debug for World<S, C>
	where
		na::DefaultAllocator: na::Allocator<i32, <C::Shape as Shape>::Dim>,
		na::DefaultAllocator: na::Allocator<i32, S::Dim>,
		C: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Wolrd")
				.field("chunks", &self.chunks)
				.finish()
		}
	}
};
