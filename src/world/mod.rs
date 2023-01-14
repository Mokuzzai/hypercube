mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;
use crate::Chunk;
use crate::Shape;
use crate::WorldCoordinate;

use std::collections::BTreeMap;

/// `N` dimensional space containing some [`Chunk`]s
///
/// * `C`: `Chunk`
/// * `E`: dimensions in the world
/// * `V`: dimensions in a chunk
pub struct World<T: Chunk<B>, const W: usize, const C: usize, const B: usize> {
	chunks: BTreeMap<OrderedVector<C>, T>,
	shape: <T as Chunk<B>>::Shape,
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> World<T, W, C, B> {
	pub fn new(shape: T::Shape) -> Self {
		Self {
			chunks: BTreeMap::new(),
			shape,
		}
	}
	pub fn chunk(&self, position: na::Vector<i32, C>) -> Option<&T> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: na::Vector<i32, C>) -> Option<&mut T> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn chunk_insert(&mut self, position: na::Vector<i32, C>, chunk: T) -> Option<T> {
		self.chunks.insert(OrderedVector::new(position), chunk)
	}
	pub fn chunk_or_insert_with(
		&mut self,
		position: na::Vector<i32, C>,
		chunk: impl FnMut() -> T,
	) -> &mut T {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&na::Vector<i32, C>, &T)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&na::Vector<i32, C>, &mut T)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &na::Vector<i32, C>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &T> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.chunks.values_mut()
	}
}
impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> World<T, W, C, B>
where
	na::Const<B>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
	na::Const<C>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
{
	pub fn world_to_chunk_block(&self, world: na::Vector<i32, W>) -> WorldCoordinate<C, B> {
		self.shape.world_to_chunk_block(world)
	}
	pub fn world_to_chunk(&self, position: na::Vector<i32, W>) -> na::Vector<i32, C> {
		self.world_to_chunk_block(position).chunk
	}
	pub fn world_to_block(&self, position: na::Vector<i32, W>) -> na::Vector<i32, B> {
		self.world_to_chunk_block(position).block
	}
	pub fn block(&mut self, position: na::Vector<i32, W>) -> Option<&T::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk(world.chunk)?.get(world.block)
	}
	pub fn block_mut(&mut self, position: na::Vector<i32, W>) -> Option<&mut T::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk_mut(world.chunk)?.get_mut(world.block)
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Default for World<T, W, C, B>
where
	<T as Chunk<B>>::Shape: Default,
{
	fn default() -> Self {
		Self::new(T::Shape::default())
	}
}

const _: () = {
	use std::fmt::*;

	impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Debug for World<T, W, C, B>
	where
		T: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Wolrd")
				.field("chunks", &self.chunks)
				.finish()
		}
	}
};

pub type UniformWorld<T, const D: usize> = World<T, D, D, D>;
