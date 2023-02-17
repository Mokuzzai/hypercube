mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::math;
use crate::Chunk;
use crate::Shape;
use crate::WorldCoordinate;

use std::collections::BTreeMap;

/// `W` dimensional space containing some [`Chunk`]s
///
/// * `T`: the type of [`Chunk`] we are storing
/// * `W`: dimensions in the world
/// * `C`: dimensions in the plane in which [`Chunk`]s are located, usually equal to `W`
/// * `B`: dimensions in a [`Chunk`]
pub struct World<T: Chunk<B>, const W: usize, const C: usize, const B: usize> {
	chunks: BTreeMap<OrderedVector<C>, T>,
	shape: <T as Chunk<B>>::Shape,
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> World<T, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn new(shape: T::Shape) -> Self {
		Self {
			chunks: BTreeMap::new(),
			shape,
		}
	}
	pub fn chunk(&self, position: math::Vector<i32, C>) -> Option<&T> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: math::Vector<i32, C>) -> Option<&mut T> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn chunk_insert(&mut self, position: math::Vector<i32, C>, chunk: T) -> Option<T> {
		self.chunks.insert(OrderedVector::new(position), chunk)
	}
	pub fn chunk_or_insert_with(
		&mut self,
		position: math::Vector<i32, C>,
		chunk: impl FnMut() -> T,
	) -> &mut T {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&math::Vector<i32, C>, &T)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&math::Vector<i32, C>, &mut T)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &math::Vector<i32, C>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &T> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.chunks.values_mut()
	}
	pub fn world_to_chunk_block(&self, world: math::Vector<i32, W>) -> WorldCoordinate<C, B> {
		self.shape.world_to_chunk_block(world)
	}
	pub fn world_to_chunk(&self, position: math::Vector<i32, W>) -> math::Vector<i32, C> {
		self.world_to_chunk_block(position).chunk
	}
	pub fn world_to_block(&self, position: math::Vector<i32, W>) -> math::Vector<i32, B> {
		self.world_to_chunk_block(position).block
	}
	pub fn block(&mut self, position: math::Vector<i32, W>) -> Option<&T::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk(world.chunk)?.get(world.block)
	}
	pub fn block_mut(&mut self, position: math::Vector<i32, W>) -> Option<&mut T::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk_mut(world.chunk)?.get_mut(world.block)
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Default for World<T, W, C, B>
where
	<T as Chunk<B>>::Shape: Default,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	fn default() -> Self {
		Self::new(T::Shape::default())
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Clone for World<T, W, C, B>
where
	<T as Chunk<B>>::Shape: Clone,
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			chunks: self.chunks.clone(),
			shape: self.shape.clone(),
		}
	}
}

const _: () = {
	use std::fmt::*;

	impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Debug for World<T, W, C, B>
	where
		T: Debug,
		T::Shape: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("World")
				.field("chunks", &self.chunks)
				.field("shape", &self.shape)
				.finish()
		}
	}
};

/// [`World`] with uniform dimensionality
pub type UniformWorld<T, const D: usize> = World<T, D, D, D>;

/// [`World`] with subuniform dimensionality
pub type SubformWorld<T, const C: usize, const B: usize> = World<T, C, C, B>;
