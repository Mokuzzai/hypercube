use super::*;

fn from_end_relative<const B: usize>(
	extents: Vector<i32, B>,
	position: Vector<i32, B>,
) -> Vector<i32, B> {
	(position + extents).zip_map(&extents, std::ops::Rem::rem)
}

pub fn world_to_chunk<const D: usize>(extents: Extents<D>, world: Position<D>) -> Position<D> {
	world_to_chunk_block(extents, world).chunk
}

pub fn world_to_block<const D: usize>(extents: Extents<D>, world: Position<D>) -> Position<D> {
	let extents = extents.cast();

	let block = world.zip_map(&extents, std::ops::Rem::rem);

	// this position might be end-relative and if it is it should be converted
	let block = from_end_relative(extents, block);

	block
}

pub fn chunk_to_world<const D: usize>(extents: Extents<D>, chunk: Position<D>) -> Position<D> {
	let extents = extents.cast();

	chunk.component_mul(&extents)
}

pub fn world_to_chunk_block<const D: usize>(
	extents: Vector<usize, D>,
	world: Vector<i32, D>,
) -> WorldCoordinate<D, D> {
	let block = world_to_block(extents, world);

	let extents = extents.cast();

	let chunk = (world - block).component_div(&extents);

	WorldCoordinate { chunk, block }
}

pub fn chunk_block_to_world<const D: usize>(
	extents: Vector<usize, D>,
	WorldCoordinate { chunk, block }: WorldCoordinate<D, D>,
) -> Vector<i32, D> {
	chunk_to_world(extents, chunk) + block
}

#[cfg(test)]
mod test {
	use crate::WorldCoordinate;

	use super::*;

	#[test]
	fn test_from_end_relative() {
		let extents = Vector::from([16; 3]);

		assert_eq!(
			Vector::from([0; 3]),
			from_end_relative(extents, Vector::from([0; 3]))
		);
		assert_eq!(
			Vector::from([0; 3]),
			from_end_relative(extents, Vector::from([-16; 3]))
		);
		assert_eq!(
			Vector::from([15; 3]),
			from_end_relative(extents, Vector::from([-1; 3]))
		);
		assert_eq!(
			Vector::from([1; 3]),
			from_end_relative(extents, Vector::from([1; 3]))
		);
		assert_eq!(
			Vector::from([0, 0, 0]),
			from_end_relative(extents, Vector::from([0, -16, 16]))
		);
	}

	fn debug_coordinates() -> [(WorldCoordinate<2, 2>, Vector<i32, 2>); 16] {
		[
			(
				WorldCoordinate::new(Vector::from([0, 0]), Vector::from([0, 0])),
				Vector::from([0, 0]),
			),
			(
				WorldCoordinate::new(Vector::from([0, 0]), Vector::from([1, 0])),
				Vector::from([1, 0]),
			),
			(
				WorldCoordinate::new(Vector::from([0, 0]), Vector::from([0, 1])),
				Vector::from([0, 1]),
			),
			(
				WorldCoordinate::new(Vector::from([0, 0]), Vector::from([1, 1])),
				Vector::from([1, 1]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, -1]), Vector::from([0, 0])),
				Vector::from([-2, -2]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, -1]), Vector::from([1, 0])),
				Vector::from([-1, -2]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, -1]), Vector::from([0, 1])),
				Vector::from([-2, -1]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, -1]), Vector::from([1, 1])),
				Vector::from([-1, -1]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, 0]), Vector::from([0, 0])),
				Vector::from([-2, 0]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, 0]), Vector::from([1, 0])),
				Vector::from([-1, 0]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, 0]), Vector::from([0, 1])),
				Vector::from([-2, 1]),
			),
			(
				WorldCoordinate::new(Vector::from([-1, 0]), Vector::from([1, 1])),
				Vector::from([-1, 1]),
			),
			(
				WorldCoordinate::new(Vector::from([0, -1]), Vector::from([0, 0])),
				Vector::from([0, -2]),
			),
			(
				WorldCoordinate::new(Vector::from([0, -1]), Vector::from([1, 0])),
				Vector::from([1, -2]),
			),
			(
				WorldCoordinate::new(Vector::from([0, -1]), Vector::from([0, 1])),
				Vector::from([0, -1]),
			),
			(
				WorldCoordinate::new(Vector::from([0, -1]), Vector::from([1, 1])),
				Vector::from([1, -1]),
			),
		]
	}

	#[test]
	fn test_world_to_chunk() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(
				world_to_chunk(Vector::from([2, 2]), world),
				chunk_block.chunk
			);
		}
	}

	#[test]
	fn test_world_to_block() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(
				world_to_block(Vector::from([2, 2]), world),
				chunk_block.block
			);
		}
	}

	#[test]
	fn test_chunk_block_to_world() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(
				chunk_block_to_world(Vector::from([2, 2]), chunk_block),
				world
			);
		}
	}
}
