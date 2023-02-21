mod imp;

use std::ops::Deref;

pub use imp::*;

pub type UniformWorldCoordinate<const D: usize> = WorldCoordinate<D, D>;
pub type WorldCoordinate<const C: usize, const B: usize> = (math::Position<C>, math::Position<B>);

use crate::math;
use crate::Positions;

pub trait Shape<const B: usize>: Sized {
	fn extents(&self) -> math::Extents< B>;

	fn positions(&self) -> Positions<B> {
		Positions::new(self.extents())
	}
	fn capacity(&self) -> usize {
		self.extents().into_iter().product()
	}
	fn position_to_index(&self, block: math::Position<B>) -> Option<usize> {
		math::position_to_index(self.extents(), block)
	}
	fn index_to_position(&self, index: usize) -> Option<math::Position<B>> {
		math::index_to_position(self.extents(), index)
	}
	fn world_to_chunk_block<const W: usize, const C: usize>(
		&self,
		world: math::Position<W>,
	) -> WorldCoordinate<C, B>
	where
		math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
		math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	{
		let (chunk, block) = math::world_to_chunk_block(
			self.extents()
				.resize_generic(math::Const::<W>, math::Const::<1>, 0),
			world.resize_generic(math::Const::<W>, math::Const::<1>, 0),
		);

		(
			chunk.resize_generic(math::Const::<C>, math::Const::<1>, 0),
			block.resize_generic(math::Const::<B>, math::Const::<1>, 0),
		)
	}
	fn chunk_block_to_world<const W: usize, const C: usize>(
		&self,
		chunk: math::Position<C>,
		block: math::Position<B>,
	) -> math::Position<W>
	where
		math::Const<B>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
		math::Const<C>: math::DimMax<math::Const<W>, Output = math::Const<W>>,
	{
		math::chunk_block_to_world(
			self.extents()
				.resize_generic(math::Const::<W>, math::Const::<1>, 0),
			chunk.resize_generic(math::Const::<W>, math::Const::<1>, 0),
			block.resize_generic(math::Const::<W>, math::Const::<1>, 0),
		)
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
	fn extents(&self) -> math::Extents<B> {
		self.deref().extents()
	}
}

impl<'a, T: UniformShape<B>, const B: usize> UniformShape<B> for Cow<'a, T> {
	fn stride(&self) -> usize {
		self.deref().stride()
	}
}
