mod imp;

use std::ops::Deref;

pub use imp::DynamicShape;
pub use imp::DynamicMultiformShape;
pub use imp::DynamicUniformShape;

use crate::na;
use crate::Positions;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WorldCoordinate<const C: usize, const B: usize> {
	pub chunk: na::Vector<i32, C>,
	pub block: na::Vector<i32, B>,
}

pub trait Shape<const B: usize>: Sized {
	fn extents(&self) -> na::Vector<usize, B>;

	fn positions(&self) -> Positions<B> {
		Positions::new(self.extents())
	}
	fn capacity(&self) -> usize {
		self.extents().into_iter().product()
	}
	fn position_to_index(&self, block: na::Vector<i32, B>) -> Option<usize> {
		na::position_to_index(self.extents(), block)
	}
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, B>> {
		na::index_to_position(self.extents(), index)
	}
	fn world_to_chunk_block<const W: usize, const C: usize>(
		&self,
		world: na::Vector<i32, W>,
	) -> WorldCoordinate<C, B>
	where
		na::Const<B>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
		na::Const<C>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
	{
		na::world_to_chunk_block(self.extents(), world)
	}
	fn chunk_block_to_world<const W: usize, const C: usize>(
		&self,
		chunk: na::Vector<i32, C>,
		block: na::Vector<i32, B>,
	) -> na::Vector<i32, W>
	where
		na::Const<B>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
		na::Const<C>: na::DimMax<na::Const<W>, Output = na::Const<W>>,
	{
		na::chunk_block_to_world(self.extents(), chunk, block)
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
	fn extents(&self) -> na::Vector<usize, B> {
		self.deref().extents()
	}
}

impl<'a, T: UniformShape<B>, const B: usize> UniformShape<B> for Cow<'a, T> {
	fn stride(&self) -> usize {
		self.deref().stride()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::multiform::CollumnChunk16x16x256;
	use crate::multiform::World2Collumns3;
	use crate::Chunk;
	use crate::WorldCoordinate;

	#[test]
	/// This test finishes `ok` but might overflow its stack
	fn test_global_to_chunk_subchunk() {
		std::thread::Builder::new()
			.name(module_path!().into())
			.stack_size(2usize.pow(26))
			.spawn(|| {
				let mut world = World2Collumns3::default();

				for y in -1..2 {
					for x in -1..2 {
						let chunk = na::Vector::from([x, y]);

						world.chunk_insert(
							na::Vector::from(chunk),
							CollumnChunk16x16x256::from_positions(|block| WorldCoordinate {
								chunk,
								block,
							}),
						);
					}
				}

				for z in 0..256 {
					for y in -16..32 {
						for x in -16..32 {
							let result = world.world_to_chunk_block(na::Vector::from([x, y, z]));

							let &expected = world
								.chunk(result.chunk)
								.unwrap()
								.get(result.block)
								.unwrap();

							assert_eq!(result, expected);
						}
					}
				}
			})
			.expect("failed to spawn thread")
			.join()
			.unwrap();
	}
}
