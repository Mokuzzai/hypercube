use crate::math;
use crate::math::Point;
use crate::storage::*;
use crate::Shape;
use crate::WorldCoordinate;
// use crate::view::View;
use crate::view::ViewMut;
use crate::view::ViewRef;

use crate::position_map::*;

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
	pub fn iter(&self) -> impl Iterator<Item = (Point<i32, C>, &T)> {
		self.inner.iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point<i32, C>, &mut T)> {
		self.inner.iter_mut()
	}
	pub fn chunk_positions(&self) -> impl Iterator<Item = (Point<i32, C>, ViewRef<T, S, B>)> {
		self.iter().map(|(p, s)| (p, ViewRef::new(s, self.shape())))
	}
	pub fn chunk_positions_mut(
		&mut self,
	) -> impl Iterator<Item = (Point<i32, C>, ViewMut<T, S, B>)> {
		self.inner
			.iter_mut()
			.map(|(p, s)| (p, ViewMut::new(s, &self.shape)))
	}
}

impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	S: Shape<B>,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn chunk_block_to_world(
		&self,
		chunk: Point<i32, C>,
		block: Point<i32, B>,
	) -> Point<i32, W> {
		self.shape.chunk_block_to_world(chunk, block)
	}
	pub fn world_to_chunk_block(&self, world: Point<i32, W>) -> WorldCoordinate<C, B> {
		self.shape.world_to_chunk_block(world)
	}
	pub fn world_to_chunk(&self, position: Point<i32, W>) -> Point<i32, C> {
		self.world_to_chunk_block(position).0
	}
	pub fn world_to_block(&self, position: Point<i32, W>) -> Point<i32, B> {
		self.world_to_chunk_block(position).1
	}
}

/// # `Chunk` | `Storage` manipulation
impl<T, S, const W: usize, const C: usize, const B: usize> Multiform<T, S, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn storage(&self, position: Point<i32, C>) -> Option<&T> {
		self.inner.get(position)
	}
	pub fn storage_mut(&mut self, position: Point<i32, C>) -> Option<&mut T> {
		self.inner.get_mut(position)
	}
	pub fn chunk(&self, position: Point<i32, C>) -> Option<ViewRef<T, S, B>> {
		self.storage(position)
			.map(|storage| ViewRef::new(storage, self.shape()))
	}
	pub fn chunk_mut(&mut self, position: Point<i32, C>) -> Option<ViewMut<T, S, B>> {
		self.inner
			.get_mut(position)
			.map(|storage| ViewMut::new(storage, &self.shape))
	}
	pub fn remove(&mut self, position: Point<i32, C>) -> Option<T> {
		self.inner.remove(position)
	}
	pub fn insert(&mut self, position: Point<i32, C>, chunk: T) -> Option<T> {
		self.inner.insert(position, chunk)
	}
	pub fn entry(&mut self, position: Point<i32, C>) -> Entry<T, C> {
		self.inner.entry(position)
	}
	pub fn storages(&self) -> impl Iterator<Item = &T> {
		self.inner.values()
	}
	pub fn storages_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.values_mut()
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

		self.storage(chunk)?.as_slice().get(index)
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

		self.storage_mut(chunk)?.as_mut_slice().get_mut(index)
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

		let index = world.shape().position_to_index(block).unwrap();

		world.entry(chunk).or_default()[index] = true;

		assert_eq!(*world.storage(math::Point::from([0; 2])).unwrap(), [true]);
	}
}
