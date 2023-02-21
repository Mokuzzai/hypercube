use super::*;

fn from_end_relative<const B: usize>(
	extents: Extents<B>,
	position: Position<B>,
) -> Position<B> {
	let extents = extents.cast();

	(position + extents).zip_map(&extents, std::ops::Rem::rem)
}

pub fn world_to_chunk<const D: usize>(extents: Extents<D>, world: Position<D>) -> Position<D> {
	world_to_chunk_block(extents, world).0
}

pub fn world_to_block<const D: usize>(extents: Extents<D>, world: Position<D>) -> Position<D> {
	let extents_i32: Position<D> = extents.cast();

	let block = world.zip_map(&extents_i32, std::ops::Rem::rem);

	// this position might be end-relative and if it is it should be converted
	let block = from_end_relative(extents, block);

	block
}

pub fn chunk_to_world<const D: usize>(extents: Extents<D>, chunk: Position<D>) -> Position<D> {
	let extents = extents.cast();

	chunk.component_mul(&extents)
}

pub fn world_to_chunk_block<const D: usize>(
	extents: Extents<D>,
	world: Position<D>,
) -> WorldCoordinate<D, D> {
	let block = world_to_block(extents, world);

	let extents = extents.cast();

	let chunk = (world - block).component_div(&extents);

	(chunk, block)
}

pub fn chunk_block_to_world<const D: usize>(
	extents: Extents<D>,
	chunk: Position<D>,
	block: Position<D>,
) -> Position<D> {
	chunk_to_world(extents, chunk) + block
}

#[cfg(test)]
mod test {
	use crate::WorldCoordinate;

	use super::*;

	#[test]
	fn test_from_end_relative() {
		let extents = Extents::from([16; 3]);

		assert_eq!(
			Position::from([0; 3]),
			from_end_relative(extents, Position::from([0; 3]))
		);
		assert_eq!(
			Position::from([0; 3]),
			from_end_relative(extents, Position::from([-16; 3]))
		);
		assert_eq!(
			Position::from([15; 3]),
			from_end_relative(extents, Position::from([-1; 3]))
		);
		assert_eq!(
			Position::from([1; 3]),
			from_end_relative(extents, Position::from([1; 3]))
		);
		assert_eq!(
			Position::from([0, 0, 0]),
			from_end_relative(extents, Position::from([0, -16, 16]))
		);
	}

	#[rustfmt::skip]
	fn debug_coordinates() -> [(WorldCoordinate<2, 2>, Position<2>); 16] {
		let f = |a, b, c| {
			(
				(Position::from(a), Position::from(b)),
				Position::from(c),
			)
		};

		[
			f([ 0,  0], [ 0,  0], [ 0,  0]),
			f([ 0,  0], [ 1,  0], [ 1,  0]),
			f([ 0,  0], [ 0,  1], [ 0,  1]),
			f([ 0,  0], [ 1,  1], [ 1,  1]),

			f([-1, -1], [ 0,  0], [-2, -2]),
			f([-1, -1], [ 1,  0], [-1, -2]),
			f([-1, -1], [ 0,  1], [-2, -1]),
			f([-1, -1], [ 1,  1], [-1, -1]),

			f([-1,  0], [ 0,  0], [-2,  0]),
			f([-1,  0], [ 1,  0], [-1,  0]),
			f([-1,  0], [ 0,  1], [-2,  1]),
			f([-1,  0], [ 1,  1], [-1,  1]),

			f([ 0, -1], [ 0,  0], [ 0, -2]),
			f([ 0, -1], [ 1,  0], [ 1, -2]),
			f([ 0, -1], [ 0,  1], [ 0, -1]),
			f([ 0, -1], [ 1,  1], [ 1, -1]),
		]
	}

	#[test]
	fn test_world_to_chunk() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(
				world_to_chunk(Extents::from([2, 2]), world),
				chunk_block.0
			);
		}
	}

	#[test]
	fn test_world_to_block() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(
				world_to_block(Extents::from([2, 2]), world),
				chunk_block.1
			);
		}
	}

	#[test]
	fn test_chunk_block_to_world() {
		for ((chunk, block), world) in debug_coordinates() {
			eprintln!("{:?} {:?} {:?}", chunk, block, world);

			assert_eq!(
				chunk_block_to_world(Extents::from([2, 2]), chunk, block),
				world
			);
		}
	}
}
