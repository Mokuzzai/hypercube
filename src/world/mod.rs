
use crate::math;
use crate::Chunk;
use crate::Shape;
use crate::WorldCoordinate;
use crate::math::Point;

use crate::position_map::*;

/// `W` dimensional space containing some [`Chunk`]s
///
/// * `T`: the type of [`Chunk`] we are storing
/// * `W`: dimensions in the world
/// * `C`: dimensions in the plane in which [`Chunk`]s are located, usually equal to `W`
/// * `B`: dimensions in a [`Chunk`])]
///
/// # Correctnes
///
/// `T.shape().extents()` must be equal to `self.shape().extents()`
///
pub struct Multiform<T: Chunk<B>, const W: usize, const C: usize, const B: usize> {
	inner: PositionMap<T, C>,
	shape: <T as Chunk<B>>::Shape,
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Multiform<T, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn new(shape: T::Shape) -> Self {
		Self {
			inner: PositionMap::new(),
			shape,
		}
	}
	pub fn shape(&self) -> &T::Shape {
		&self.shape
	}
	pub fn len(&self) -> usize {
		self.inner.len()
	}
	pub fn positions(&self) -> impl '_ + Iterator<Item = Point<i32, C>> {
		self.inner.positions()
	}
	pub fn chunk_block_to_world(&self, chunk: Point<i32, C>, block: Point<i32, B>) -> Point<i32, W> {
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
	pub fn iter(&self) -> impl Iterator<Item = (Point<i32, C>, &T)> {
		self.inner.iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point<i32, C>, &mut T)> {
		self.inner.iter_mut()
	}
}

/// # `Chunk` manipulation
impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Multiform<T, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn chunk(&self, position: Point<i32, C>) -> Option<&T> {
		self.inner.get(position)
	}
	pub fn chunk_mut(&mut self, position: Point<i32, C>) -> Option<&mut T> {
		self.inner.get_mut(position)
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
	pub fn chunks(&self) -> impl Iterator<Item = &T> {
		self.inner.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.values_mut()
	}
}

/// # Block manipulation
impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Multiform<T, W, C, B>
where
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	pub fn block(&self, chunk: math::Point<i32, C>, block: math::Point<i32, B>) -> Option<&T::Item> {
		self.chunk(chunk)?.get(block)
	}
	pub fn block_mut(&mut self, chunk: Point<i32, C>, block: Point<i32, B>) -> Option<&mut T::Item> {
		self.chunk_mut(chunk)?.get_mut(block)
	}
	pub fn get_block(&self, position: Point<i32, W>) -> Option<&T::Item> {
		let (chunk, block) = self.world_to_chunk_block(position);

		self.block(chunk, block)
	}
	pub fn get_block_mut(&mut self, position: Point<i32, W>) -> Option<&mut T::Item> {
		let (chunk, block) = self.world_to_chunk_block(position);

		self.block_mut(chunk, block)
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Default for Multiform<T, W, C, B>
where
	<T as Chunk<B>>::Shape: Default,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
	fn default() -> Self {
		Self::new(T::Shape::default())
	}
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Eq for Multiform<T, W, C, B>
where
	T: Eq,
	T::Shape: Eq,
	math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
{
}

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> PartialEq for Multiform<T, W, C, B>
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

impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Clone for Multiform<T, W, C, B>
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

	impl<T: Chunk<B>, const W: usize, const C: usize, const B: usize> Debug for Multiform<T, W, C, B>
	where
		T: Debug,
		T::Shape: Debug,
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
pub type Uniform<T, const D: usize> = Multiform<T, D, D, D>;

/// `World` with subuniform dimensionality
pub type Subform<T, const C: usize, const B: usize> = Multiform<T, B, C, B>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Array;
	use crate::ct;

	#[test]
	fn test_get_block_mut_else_default() {
		let mut world = Uniform::<Array<bool, ct::Uniform<1, 2>, 2, 1>, 2>::default();

		let (chunk, block) = world.world_to_chunk_block(math::Point::from([0; 2]));

		*world.entry(chunk).or_default().block_mut(block) = true;

		assert_eq!(*world.chunk(math::Point::from([0; 2])).unwrap(), Array::from_buffer([true]));
	}
}
