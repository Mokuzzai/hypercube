pub mod entry;

pub use entry::Entry;

use crate::chunk::Chunk;
use crate::chunk::ChunkMut;
use crate::chunk::ChunkRef;
use crate::math;
use crate::math::Coordinate;
use crate::math::Point;
use crate::storage::*;
use crate::Shape;
use crate::WorldCoordinate;

use crate::position_map::PositionMap;

/// `W` dimensional space containing some `Chunk`s
///
/// * `T`: the [`Storage`](crate::storage::Storage) type
/// * `S`: the `Chunk`s [`Shape`]
/// * `W`: dimensions in the world
/// * `C`: dimensions in the plane in which `Chunk`s are located, usually equal to `W`
/// * `B`: dimensions in a `Chunk`
///
pub struct Multiform<T, S, const W: usize, const C: usize, const B: usize> {
	inner: PositionMap<T, C>,
	shape: S,
}

impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn new(shape: S) -> Self {
		Self {
			inner: PositionMap::new(),
			shape,
		}
	}
	pub fn shape(&self) -> &S {
		&self.shape
	}
	pub fn len(&self) -> usize {
		self.inner.len()
	}
	pub fn positions(&self) -> impl '_ + Iterator<Item = Point<i32, C>> {
		self.inner.positions()
	}
}

impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	S: Shape<B>,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn iter(&self) -> impl Iterator<Item = (Point<i32, C>, ChunkRef<T, S, B>)> {
		self.inner
			.iter()
			.map(|(p, s)| (p, ChunkRef::new(s, self.shape)))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point<i32, C>, ChunkMut<T, S, B>)> {
		self.inner
			.iter_mut()
			.map(|(p, s)| (p, ChunkMut::new(s, self.shape)))
	}
	pub fn chunks(&self) -> impl Iterator<Item = ChunkRef<T, S, B>> {
		self.inner.values().map(|s| ChunkRef::new(s, self.shape))
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = ChunkMut<T, S, B>> {
		self.inner
			.values_mut()
			.map(|s| ChunkMut::new(s, self.shape))
	}
	pub fn chunk_block_to_world<N: Coordinate>(
		&self,
		chunk: Point<N, C>,
		block: Point<N, B>,
	) -> Point<N, W> {
		self.shape.chunk_block_to_world(chunk, block)
	}
	pub fn world_to_chunk_block<N: Coordinate>(
		&self,
		world: Point<N, W>,
	) -> WorldCoordinate<N, C, B> {
		self.shape.world_to_chunk_block(world)
	}
	pub fn world_to_chunk<N: Coordinate>(&self, position: Point<N, W>) -> Point<N, C> {
		self.world_to_chunk_block(position).0
	}
	pub fn world_to_block<N: Coordinate>(&self, position: Point<N, W>) -> Point<N, B> {
		self.world_to_chunk_block(position).1
	}
}

/// # `Chunk` | `Storage` manipulation
impl<T, S: Shape<B>, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn chunk(&self, position: Point<i32, C>) -> Option<ChunkRef<T, S, B>> {
		self.inner
			.get(position)
			.map(|storage| ChunkRef::new(storage, self.shape))
	}
	pub fn chunk_mut(&mut self, position: Point<i32, C>) -> Option<ChunkMut<T, S, B>> {
		self.inner
			.get_mut(position)
			.map(|storage| ChunkMut::new(storage, self.shape))
	}
	pub fn remove(&mut self, position: Point<i32, C>) -> Option<Chunk<T, S, B>> {
		self.inner
			.remove(position)
			.map(|storage| Chunk::new(storage, self.shape))
	}
	/// # Panics
	/// This function panics if `chunk.position != self.position`
	pub fn insert(&mut self, position: Point<i32, C>, chunk: Chunk<T, S, B>) -> Option<T> {
		let (shape, storage) = chunk.into_raw_parts();

		assert!(shape == self.shape);

		self.inner.insert(position, storage)
	}
	pub fn entry(&mut self, position: Point<i32, C>) -> Entry<T, S, C, B> {
		Entry::from(self.inner.entry(position), self.shape)
	}
}

/// # Block manipulation
impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	T: ContiguousMemory,
	S: Shape<B>,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn block(
		&self,
		chunk: math::Point<i32, C>,
		block: math::Point<i32, B>,
	) -> Option<&T::Item> {
		let index = self.shape.position_to_index(block)?;

		self.inner.get(chunk)?.as_slice().get(index)
	}
	pub fn get_block(&self, position: Point<i32, W>) -> Option<&T::Item> {
		let (chunk, block) = self.world_to_chunk_block(position);

		self.block(chunk, block)
	}
}
impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	T: ContiguousMemoryMut,
	S: Shape<B>,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn block_mut(
		&mut self,
		chunk: Point<i32, C>,
		block: Point<i32, B>,
	) -> Option<&mut T::Item> {
		let index = self.shape.position_to_index(block)?;

		self.inner.get_mut(chunk)?.as_mut_slice().get_mut(index)
	}
	pub fn get_block_mut(&mut self, position: Point<i32, W>) -> Option<&mut T::Item> {
		let (chunk, block) = self.world_to_chunk_block(position);

		self.block_mut(chunk, block)
	}
}

impl<T, S, const W: usize, const C: usize, const B: usize> Default for Multiform<T, S, W, C, B>
where
	S: Default,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	fn default() -> Self {
		Self::new(S::default())
	}
}

impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	T: Eq,
	S: Eq,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
}

impl<T, S, const W: usize, const C: usize, const B: usize> PartialEq for Multiform<T, S, W, C, B>
where
	T: PartialEq,
	S: PartialEq,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	fn eq(&self, other: &Self) -> bool {
		self.shape == other.shape && self.inner == other.inner
	}
}

impl<T, S, const W: usize, const C: usize, const B: usize> Clone for Multiform<T, S, W, C, B>
where
	S: Clone,
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			shape: self.shape.clone(),
		}
	}
}

const _: () = {
	use std::fmt::*;

	impl<T, S, const W: usize, const C: usize, const B: usize> Debug for Multiform<T, S, W, C, B>
	where
		T: Debug,
		S: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("World")
				.field("inner", &self.inner)
				.field("shape", &self.shape)
				.finish()
		}
	}
};

/// `World` with uniform dimensionality
pub type Uniform<T, S, const D: usize> = Multiform<T, S, D, D, D>;

/// `World` with subuniform dimensionality
pub type Subform<T, S, const C: usize, const B: usize> = Multiform<T, S, B, C, B>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ct;

	#[test]
	fn test_get_block_mut_else_default() {
		let mut world = Uniform::<[bool; 1], ct::Uniform<1, 2>, 2>::default();

		let (chunk, block) = world.world_to_chunk_block(math::Point::from([0; 2]));

		world
			.entry(chunk)
			.or_default()
			.replace(block, true)
			.unwrap();

		assert_eq!(
			**world.chunk(math::Point::from([0; 2])).unwrap().storage(),
			[true]
		);
	}
}
