mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::na;
use crate::nd;

use crate::Chunk;

use std::collections::BTreeMap;

/// * `T`: [`Chunk`] type
/// * `C`: [`Chunk`] space dimensions
/// * `B`: `Block` space dimensions
#[derive(Clone)]
pub struct World<T, const C: usize, const B: usize> {
	chunks: BTreeMap<OrderedVector<C>, T>,
	chunk_stride: usize,
}

impl<T, const C: usize, const B: usize> World<T, C, B> {
	pub fn new(chunk_stride: usize) -> Self {
		Self {
			chunks: BTreeMap::new(),
			chunk_stride,
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
	) -> &T {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WorldCoordinate<const C: usize, const B: usize> {
	pub chunk: na::Vector<i32, C>,
	pub block: na::Vector<i32, B>,
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> World<T, C, B>
where
	na::Const<C>: na::DimMax<na::Const<B>, Output = na::Const<W>>,
	[usize; B]: ndarray::Dimension,
{
	fn world_to_chunk_block(
		&self,
		world: na::Vector<i32, W>,
	) -> WorldCoordinate<C, B>
	{
		let chunk_shape = na::Vector::from([self.chunk_stride; C]).cast::<i32>();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		// this subchunk might be negative and if it is it should be inversed
		let mut block_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Rem::rem);

		for (value, &extent) in block_as_global.iter_mut().zip(chunk_shape_as_global.iter()) {
			*value = (*value + extent) % extent
		}

		let chunk_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Div::div);

		let chunk = chunk_as_global.resize_generic(na::Const::<C>, na::Const::<1>, 0);
		let block = block_as_global.resize_generic(na::Const::<B>, na::Const::<1>, 0);

		WorldCoordinate { chunk, block }
	}
	fn chunk_block_to_world(
		&self,
		chunk: na::Vector<i32, C>,
		block: na::Vector<i32, B>,
	) -> na::Vector<i32, W> {
		let chunk_shape = na::Vector::from([self.chunk_stride; C]).cast::<i32>();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		let chunk_as_global = chunk.resize_generic(na::Const::<W>, na::Const::<1>, 0);
		let block_as_global = block.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		chunk_as_global + block_as_global.component_mul(&chunk_shape_as_global)
	}
	pub fn world_to_chunk(&self, position: na::Vector<i32, W>) -> na::Vector<i32, C> {
		self.world_to_chunk_block(position).chunk
	}
	pub fn world_to_block(&self, position: na::Vector<i32, W>) -> na::Vector<i32, B> {
		self.world_to_chunk_block(position).block
	}
	pub fn block(&mut self, position: na::Vector<i32, W>) -> Option<&T::Block> {
		let world = self.world_to_chunk_block(position);

		self.chunk(world.chunk)?.get(world.block)
	}
	pub fn block_mut(&mut self, position: na::Vector<i32, W>) -> Option<&mut T::Block> {
		let world = self.world_to_chunk_block(position);

		self.chunk_mut(world.chunk)?.get_mut(world.block)
	}
}

impl<T, const C: usize, const B: usize> Default for World<T, C, B> {
	fn default() -> Self {
		Self {
			chunks: Default::default(),
			chunk_stride: Default::default(),
		}
	}
}
const _: () = {
	use std::fmt::*;

	impl<T, const C: usize, const B: usize> Debug for World<T, C, B>
	where
		T: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("World")
				.finish_non_exhaustive()
		}
	}
};








