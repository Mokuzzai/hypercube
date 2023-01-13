mod ordered_vector;

use ordered_vector::OrderedVector;

use crate::WorldCoordinate;
use crate::na;
use crate::Chunk;
use crate::Shape;

use std::collections::BTreeMap;

/// `N` dimensional space containing some [`Chunk`]s
///
/// * `C`: `Chunk`
/// * `E`: dimensions in the world
/// * `V`: dimensions in a chunk
pub struct World<C: Chunk<V>, const E: usize, const V: usize> {
	chunks: BTreeMap<OrderedVector<E>, C>,
}

impl<C: Chunk<V>, const E: usize, const V: usize> World<C, E, V> {
	pub fn new() -> Self {
		Self {
			chunks: BTreeMap::new(),
		}
	}
	pub fn chunk(&self, position: na::Vector<i32, E>) -> Option<&C> {
		self.chunks.get(&OrderedVector::new(position))
	}
	pub fn chunk_mut(&mut self, position: na::Vector<i32, E>) -> Option<&mut C> {
		self.chunks.get_mut(&OrderedVector::new(position))
	}
	pub fn chunk_insert(&mut self, position: na::Vector<i32, E>, chunk: C) -> Option<C> {
		self.chunks.insert(OrderedVector::new(position), chunk)
	}
	pub fn chunk_or_insert_with(
		&mut self,
		position: na::Vector<i32, E>,
		chunk: impl FnMut() -> C,
	) -> &mut C {
		self.chunks
			.entry(OrderedVector::new(position))
			.or_insert_with(chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&na::Vector<i32, E>, &C)> {
		self.chunks.iter().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (&na::Vector<i32, E>, &mut C)> {
		self.chunks.iter_mut().map(|(a, b)| (&a.coordinates, b))
	}
	pub fn positions(&self) -> impl Iterator<Item = &na::Vector<i32, E>> {
		self.chunks.keys().map(|a| &a.coordinates)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &C> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut C> {
		self.chunks.values_mut()
	}
}

impl<C: Chunk<V>, const E: usize, const V: usize, const W: usize> World<C, E, V>
where
	na::Const<E>: na::DimMax<na::Const<V>, Output = na::Const<W>>,
{
	pub fn world_to_chunk_block(
		&self,
		world: na::Vector<i32, W>,
	) -> WorldCoordinate<E, V> {
		C::SHAPE.world_to_chunk_block(world)
	}
	pub fn world_to_chunk(&self, position: na::Vector<i32, W>) -> na::Vector<i32, E> {
		self.world_to_chunk_block(position).chunk
	}
	pub fn world_to_block(&self, position: na::Vector<i32, W>) -> na::Vector<i32, V> {
		self.world_to_chunk_block(position).block
	}
	pub fn block(&mut self, position: na::Vector<i32, W>) -> Option<&C::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk(world.chunk)?.get(world.block)
	}
	pub fn block_mut(&mut self, position: na::Vector<i32, W>) -> Option<&mut C::Item> {
		let world = self.world_to_chunk_block(position);

		self.chunk_mut(world.chunk)?.get_mut(world.block)
	}
}

impl<C: Chunk<V>, const E: usize, const V: usize> Default for World<C, E, V> {
	fn default() -> Self {
		Self::new()
	}
}

const _: () = {
	use std::fmt::*;

	impl<C: Chunk<V>, const E: usize, const V: usize> Debug for World<C, E, V>
	where
		C: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Wolrd")
				.field("chunks", &self.chunks)
				.finish()
		}
	}
};

#[cfg(test)]
mod tests {
	use super::*;
	use crate::multiform::CollumnChunk16x16x256;
	use crate::multiform::World2Collumns3;
	use crate::WorldCoordinate;

	#[test]
	/// This test finishes `ok` but might overflow its stack
	fn test_global_to_chunk_subchunk() {
		std::thread::Builder::new()
			.name(module_path!().into())
			.stack_size(2usize.pow(26))
			.spawn(|| {
				let mut world = World2Collumns3::new();

				for y in -1..2 {
					for x in -1..2 {
						let chunk = na::Vector::from([x, y]);

						world.chunk_insert(
							na::Vector::from(chunk),
							CollumnChunk16x16x256::from_positions(|block| WorldCoordinate { chunk, block }),
						);
					}
				}

				for z in 0..256 {
					for y in -16..32 {
						for x in -16..32 {
							let result =
								world.world_to_chunk_block(na::Vector::from([x, y, z]));

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
