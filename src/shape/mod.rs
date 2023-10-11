mod imp;

use math::*;

// use std::ops::Deref;

pub use imp::*;

use crate::math;

pub(crate) type UniformWorldCoordinate<S, const D: usize> = WorldCoordinate<S, D, D>;
pub(crate) type WorldCoordinate<S, const C: usize, const B: usize> = (Point<S, C>, Point<S, B>);

pub trait Shape<const B: usize>: Sized + Copy + Eq {
	fn extents(&self) -> Vector<usize, B>;

	#[inline(always)]
	fn capacity(&self) -> usize {
		self.extents().into_iter().product()
	}
	#[inline(always)]
	fn position_to_index(&self, block: Point<i32, B>) -> Option<usize> {
		position_to_index(self.extents(), block)
	}
	#[inline(always)]
	fn index_to_position(&self, index: usize) -> Option<Point<i32, B>> {
		index_to_position(self.extents(), index)
	}
	#[inline(always)]
	fn world_to_chunk_block<S: Coordinate, const W: usize, const C: usize>(
		&self,
		world: Point<S, W>,
	) -> WorldCoordinate<S, C, B>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		let (chunk, block) = world_to_chunk_block(
			self.extents().resize_generic(Const::<W>, Const::<1>, 0),
			world
				.coords
				.resize_generic(Const::<W>, Const::<1>, S::zero())
				.into(),
		);

		(
			chunk
				.coords
				.resize_generic(Const::<C>, Const::<1>, S::zero())
				.into(),
			block
				.coords
				.resize_generic(Const::<B>, Const::<1>, S::zero())
				.into(),
		)
	}
	#[inline(always)]
	fn chunk_block_to_world<S: Coordinate, const W: usize, const C: usize>(
		&self,
		chunk: Point<S, C>,
		block: Point<S, B>,
	) -> Point<S, W>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		chunk_block_to_world(
			self.extents().resize_generic(Const::<W>, Const::<1>, 0),
			chunk
				.coords
				.resize_generic(Const::<W>, Const::<1>, S::zero())
				.into(),
			block
				.coords
				.resize_generic(Const::<W>, Const::<1>, S::zero())
				.into(),
		)
	}
	#[inline(always)]
	fn world_to_chunk<S: Coordinate, const W: usize, const C: usize>(
		&self,
		position: Point<S, W>,
	) -> Point<S, C>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		self.world_to_chunk_block(position).0
	}
	#[inline(always)]
	fn world_to_block<S: Coordinate, const W: usize>(&self, position: Point<S, W>) -> Point<S, B>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
	{
		self.world_to_chunk_block(position).1
	}
}

pub trait UniformShape<const B: usize>: Shape<B> {
	fn stride(&self) -> usize;
}

impl<'a, T: Shape<B>, const B: usize> Shape<B> for &'a T {
	#[inline(always)]
	fn extents(&self) -> math::Vector<usize, B> {
		T::extents(&**self)
	}

	#[inline(always)]
	fn capacity(&self) -> usize {
		T::capacity(&**self)
	}
	#[inline(always)]
	fn position_to_index(&self, block: Point<i32, B>) -> Option<usize> {
		T::position_to_index(&**self, block)
	}
	#[inline(always)]
	fn index_to_position(&self, index: usize) -> Option<Point<i32, B>> {
		T::index_to_position(&**self, index)
	}
	#[inline(always)]
	fn world_to_chunk_block<S: Coordinate, const W: usize, const C: usize>(
		&self,
		world: Point<S, W>,
	) -> WorldCoordinate<S, C, B>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		T::world_to_chunk_block(&**self, world)
	}
	#[inline(always)]
	fn chunk_block_to_world<S: Coordinate, const W: usize, const C: usize>(
		&self,
		chunk: Point<S, C>,
		block: Point<S, B>,
	) -> Point<S, W>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		T::chunk_block_to_world(&**self, chunk, block)
	}
	#[inline(always)]
	fn world_to_chunk<S: Coordinate, const W: usize, const C: usize>(
		&self,
		position: Point<S, W>,
	) -> Point<S, C>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
		Const<C>: DimMax<Const<W>, Output = Const<W>>,
	{
		T::world_to_chunk(&**self, position)
	}
	#[inline(always)]
	fn world_to_block<S: Coordinate, const W: usize>(&self, position: Point<S, W>) -> Point<S, B>
	where
		Const<B>: DimMax<Const<W>, Output = Const<W>>,
	{
		T::world_to_block(&**self, position)
	}
}

impl<'a, T: UniformShape<B>, const B: usize> UniformShape<B> for &'a T {
	fn stride(&self) -> usize {
		T::stride(&**self)
	}
}
