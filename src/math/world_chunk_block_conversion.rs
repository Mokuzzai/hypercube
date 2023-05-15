use super::*;

#[inline(always)]
fn from_end_relative<S: Coordinate, const B: usize>(
	extents: Vector<usize, B>,
	position: Point<S, B>,
) -> Point<S, B> {
	let extents = matrix_cast(extents).unwrap();

	(position + &extents)
		.coords
		.zip_map(&extents, std::ops::Rem::rem)
		.into()
}

#[inline(always)]
pub fn world_to_chunk<S: Coordinate, const D: usize>(
	extents: Vector<usize, D>,
	world: Point<S, D>,
) -> Point<S, D> {
	world_to_chunk_block(extents, world).0
}

#[inline(always)]
pub fn world_to_block<S: Coordinate, const D: usize>(
	extents: Vector<usize, D>,
	world: Point<S, D>,
) -> Point<S, D> {
	let extents_i32: Vector<S, D> = matrix_cast(extents).unwrap();

	let block = world
		.coords
		.zip_map(&extents_i32, std::ops::Rem::rem)
		.into();

	// this position might be end-relative and if it is it should be converted
	let block = from_end_relative(extents, block);

	block
}

#[inline(always)]
pub fn chunk_to_world<S: Coordinate, const D: usize>(
	extents: Vector<usize, D>,
	chunk: Point<S, D>,
) -> Point<S, D> {
	let extents = matrix_cast(extents).unwrap();

	chunk.coords.component_mul(&extents).into()
}

#[inline(always)]
pub fn world_to_chunk_block<S: Coordinate, const D: usize>(
	extents: Vector<usize, D>,
	world: Point<S, D>,
) -> UniformWorldCoordinate<S, D> {
	let block = world_to_block(extents, world);

	let extents = matrix_cast(extents).unwrap();

	let chunk = (world - block).component_div(&extents).into();

	(chunk, block)
}

#[inline(always)]
pub fn chunk_block_to_world<S: Coordinate, const D: usize>(
	extents: Vector<usize, D>,
	chunk: Point<S, D>,
	block: Point<S, D>,
) -> Point<S, D> {
	chunk_to_world(extents, chunk) + block.coords
}

#[cfg(test)]
mod test {
	use crate::UniformWorldCoordinate;

	use super::*;

	#[test]
	fn test_from_end_relative() {
		let extents = Vector::from([16; 3]);

		assert_eq!(
			Point::from([0; 3]),
			from_end_relative(extents, Point::from([0; 3]))
		);
		assert_eq!(
			Point::from([0; 3]),
			from_end_relative(extents, Point::from([-16; 3]))
		);
		assert_eq!(
			Point::from([15; 3]),
			from_end_relative(extents, Point::from([-1; 3]))
		);
		assert_eq!(
			Point::from([1; 3]),
			from_end_relative(extents, Point::from([1; 3]))
		);
		assert_eq!(
			Point::from([0, 0, 0]),
			from_end_relative(extents, Point::from([0, -16, 16]))
		);
	}

	#[rustfmt::skip]
	fn debug_coordinates() -> [(UniformWorldCoordinate<i32, 2>, Point<i32, 2>); 16] {
		let f = |a, b, c| {
			(
				(Point::from(a), Point::from(b)),
				Point::from(c),
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

			assert_eq!(world_to_chunk(Vector::from([2, 2]), world), chunk_block.0);
		}
	}

	#[test]
	fn test_world_to_block() {
		for (chunk_block, world) in debug_coordinates() {
			eprintln!("{:?} {:?}", chunk_block, world);

			assert_eq!(world_to_block(Vector::from([2, 2]), world), chunk_block.1);
		}
	}

	#[test]
	fn test_chunk_block_to_world() {
		for ((chunk, block), world) in debug_coordinates() {
			eprintln!("{:?} {:?} {:?}", chunk, block, world);

			assert_eq!(
				chunk_block_to_world(Vector::from([2, 2]), chunk, block),
				world
			);
		}
	}
}
