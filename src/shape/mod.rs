mod imp;

use std::ops::Deref;

pub use imp::*;

use crate::math;
use crate::Positions;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WorldCoordinate<const C: usize, const B: usize> {
	pub chunk: math::Vector<i32, C>,
	pub block: math::Vector<i32, B>,
}

pub trait Shape<const B: usize>: Sized {
	fn extents(&self) -> math::Vector<usize, B>;

	fn positions(&self) -> Positions<B> {
		Positions::new(self.extents())
	}
	fn capacity(&self) -> usize {
		self.extents().into_iter().product()
	}
	fn position_to_index(&self, block: math::Vector<i32, B>) -> Option<usize> {
		math::position_to_index(self.extents(), block)
	}
	fn index_to_position(&self, index: usize) -> Option<math::Vector<i32, B>> {
		math::index_to_position(self.extents(), index)
	}
	fn world_to_chunk_block<const W: usize, const C: usize>(
		&self,
		world: math::Vector<i32, W>,
	) -> WorldCoordinate<C, B>
	where
		math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
		math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	{
		math::world_to_chunk_block(self.extents(), world)
	}
	fn chunk_block_to_world<const W: usize, const C: usize>(
		&self,
		world: WorldCoordinate<C, B>,
	) -> math::Vector<i32, W>
	where
		math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
		math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	{
		math::chunk_block_to_world(self.extents(), world)
	}
}

pub trait UniformShape<const B: usize>: Shape<B> {
	fn stride(&self) -> usize;
}

#[derive(Debug)]
pub enum Cow<'a, T> {
	Owned(T),
	Borrowed(&'a T),
}

impl<'a, T> Deref for Cow<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match *self {
			Self::Owned(ref t) => t,
			Self::Borrowed(t) => t,
		}
	}
}

impl<'a, T: Shape<B>, const B: usize> Shape<B> for Cow<'a, T> {
	fn extents(&self) -> math::Vector<usize, B> {
		self.deref().extents()
	}
}

impl<'a, T: UniformShape<B>, const B: usize> UniformShape<B> for Cow<'a, T> {
	fn stride(&self) -> usize {
		self.deref().stride()
	}
}
