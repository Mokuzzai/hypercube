mod entry;
mod ordered_vector;

pub use entry::Entry;
pub use entry::OccupiedEntry;
pub use entry::VacantEntry;

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
/// * `B`: dimensions in a [`Chunk`])]
pub struct World<T: Chunk<B>, const W: usize, const C: usize, const B: usize> {
	inner: BTreeMap<OrderedVector<C>, T>,
	shape: <T as Chunk<B>>::Shape,
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> World<T, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn new(shape: T::Shape) -> Self {
		Self {
			inner: BTreeMap::new(),
			shape,
		}
	}
	pub fn shape(&self) -> &T::Shape {
		&self.shape
	}
	pub fn chunk(&self, position: math::Vector<i32, C>) -> Option<&T> {
		self.inner.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: math::Vector<i32, C>) -> Option<&mut T> {
		self.inner.get_mut(&OrderedVector::new(position))
	}
	pub fn get_chunk(&self, position: math::Vector<i32, W>) -> Option<&T> {
		let chunk = self.world_to_chunk(position);

		self.chunk(chunk)
	}
	pub fn get_chunk_mut(&mut self, position: math::Vector<i32, W>) -> Option<&mut T> {
		let chunk = self.world_to_chunk(position);

		self.chunk_mut(chunk)
	}
	pub fn insert(&mut self, position: math::Vector<i32, C>, chunk: T) -> Option<T> {
		self.inner.insert(OrderedVector::new(position), chunk)
	}
	pub fn entry(&mut self, position: math::Vector<i32, C>) -> Entry<T, C> {
		let entry = self.inner.entry(OrderedVector::new(position));

		Entry::from(entry)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&math::Vector<i32, C>, &T)> {
		self.inner.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&math::Vector<i32, C>, &mut T)> {
		self.inner.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &math::Vector<i32, C>> {
		self.inner.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &T> {
		self.inner.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.values_mut()
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
	pub fn block(&self, position: WorldCoordinate<C, B>) -> Option<&T::Item> {
		self.chunk(position.chunk)?.get(position.block)
	}
	pub fn block_mut(&mut self, position: WorldCoordinate<C, B>) -> Option<&mut T::Item> {
		self.chunk_mut(position.chunk)?.get_mut(position.block)
	}
	pub fn get_block(&self, position: math::Vector<i32, W>) -> Option<&T::Item> {
		let world = self.world_to_chunk_block(position);

		self.block(world)
	}
	pub fn get_block_mut(&mut self, position: math::Vector<i32, W>) -> Option<&mut T::Item> {
		let world = self.world_to_chunk_block(position);

		self.block_mut(world)
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

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Eq for World<T, W, C, B>
where
	T: Eq,
	T::Shape: Eq,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> PartialEq for World<T, W, C, B>
where
	T: PartialEq,
	T::Shape: PartialEq,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	fn eq(&self, other: &Self) -> bool {
		self.shape == other.shape && self.inner == other.inner
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Clone for World<T, W, C, B>
where
	<T as Chunk<B>>::Shape: Clone,
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

	impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Debug for World<T, W, C, B>
	where
		T: Debug,
		T::Shape: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("World")
				.field("chunks", &self.inner)
				.field("shape", &self.shape)
				.finish()
		}
	}
};

/// [`World`] with uniform dimensionality
pub type Uniform<T, const D: usize> = World<T, D, D, D>;

/// [`World`] with subuniform dimensionality
pub type Subform<T, const C: usize, const B: usize> = World<T, B, C, B>;
