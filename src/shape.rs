use crate::na;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WorldCoordinate<const C: usize, const B: usize> {
	pub chunk: na::Vector<i32, C>,
	pub block: na::Vector<i32, B>
}

pub trait Shape<const B: usize> {
	fn new() -> Self;

	/// Must be nonzero
	fn extents(&self) -> na::Vector<i32, B>;

	fn capacity(&self) -> usize {
		self.extents().into_iter().map(|&c| usize::try_from(c).unwrap()).product()
	}

	fn position_to_index(&self, block: na::Vector<i32, B>) -> Option<usize> {
		crate::position_index_conversion::multiform::position_to_index(
			na::itou(na::vtoa(self.extents())).unwrap(),
			na::itou(na::vtoa(block)).unwrap(),
		)
	}
	fn index_to_position(&self, index: usize) -> Option<na::Vector<i32, B>> {
		let src = crate::position_index_conversion::multiform::index_to_position(na::itou(na::vtoa(self.extents())).unwrap(), index)?;

		Some(na::atov(na::utoi(src).unwrap()))
	}
	fn world_to_chunk_block<const W: usize, const C: usize>(
		&self,
		world: na::Vector<i32, W>,
	) -> WorldCoordinate<C, B>
	where
		na::Const<C>: na::DimMax<na::Const<B>, Output = na::Const<W>>,
	{
		let chunk_shape = self.extents();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		// this subchunk might be negative and if it is it should be inversed
		let mut block_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Rem::rem);

		for (value, &extent) in block_as_global
			.iter_mut()
			.zip(chunk_shape_as_global.iter())
		{
			*value = (*value + extent) % extent
		}

		let chunk_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Div::div);

		let chunk = chunk_as_global.resize_generic(na::Const::<C>, na::Const::<1>, 0);
		let block = block_as_global.resize_generic(na::Const::<B>, na::Const::<1>, 0);

		WorldCoordinate { chunk, block }
	}
	fn chunk_block_to_world<const W: usize, const C: usize, >(&self, chunk: na::Vector<i32, C>, block: na::Vector<i32, B>) -> na::Vector<i32, W>
	where
		na::Const<C>: na::DimMax<na::Const<B>, Output = na::Const<W>>,
	{
		let chunk_shape = self.extents();

		let chunk_shape_as_global = chunk_shape.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		let chunk_as_global = chunk.resize_generic(na::Const::<W>, na::Const::<1>, 0);
		let block_as_global = block.resize_generic(na::Const::<W>, na::Const::<1>, 0);

		chunk_as_global + block_as_global.component_mul(&chunk_shape_as_global)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::multiform::CollumnChunk16x16x256;
	use crate::multiform::World2Collumns3;
	use crate::WorldCoordinate;
	use crate::Chunk;

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
