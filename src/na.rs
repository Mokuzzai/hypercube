pub use nalgebra::dimension::DimMax;
pub use nalgebra::dimension::DimMaximum;
pub use nalgebra::Dim;

pub type Vector<T, const D: usize> = OVector<T, Const<D>>;

pub use nalgebra::OVector;

pub use nalgebra::dimension::Const;

pub use nalgebra::Scalar;

use crate::WorldCoordinate;

pub fn position_to_index<const B: usize>(
	extents: Vector<usize, B>,
	position: Vector<i32, B>,
) -> Option<usize> {
	(0..B).try_fold(0, |acc, i| {
		let stride = extents[i];

		let coordinate: usize = position[i].try_into().ok()?;

		if coordinate >= stride {
			return None;
		}

		let subd: usize = extents.into_iter().take(i).product();

		Some(acc + coordinate * subd)
	})
}

pub fn index_to_position<const B: usize>(
	extents: Vector<usize, B>,
	index: usize,
) -> Option<Vector<i32, B>> {
	let capacity = extents.into_iter().product();

	let extents_i32: Vector<i32, B> = extents.cast();

	if index >= capacity {
		return None;
	}

	let mut out = Vector::from_element(0i32);

	let prev = 0;

	for i in 0..B {
		let subd: i32 = extents_i32.into_iter().take(i).product();

		out[i] = (index as i32 - prev) / subd % extents_i32[i]
	}

	Some(out)
}

pub fn position_to_index_offset<const B: usize>(
	extents: Vector<usize, B>,
	offset: Vector<i32, B>,
	position: Vector<i32, B>,
) -> Option<usize> {
	position_to_index(extents, position - offset)
}

pub fn index_to_position_offset<const B: usize>(
	extents: Vector<usize, B>,
	offset: Vector<i32, B>,
	index: usize,
) -> Option<Vector<i32, B>> {
	index_to_position(extents, index).map(|position| position + offset)
}

pub fn world_to_chunk_block<const W: usize, const C: usize, const B: usize>(
	extents: Vector<usize, B>,
	world: Vector<i32, W>,
) -> WorldCoordinate<C, B>
where
	Const<B>: DimMax<Const<W>, Output = Const<W>>,
	Const<C>: DimMax<Const<W>, Output = Const<W>>,
{
	let chunk_shape = extents.cast::<i32>();

	let chunk_shape_as_global = chunk_shape.resize_generic(Const::<W>, Const::<1>, 0);

	// this subchunk might be negative and if it is it should be inversed
	let mut block_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Rem::rem);

	for (value, &extent) in block_as_global.iter_mut().zip(chunk_shape_as_global.iter()) {
		*value = (*value + extent) % extent
	}

	let chunk_as_global = world.zip_map(&chunk_shape_as_global, std::ops::Div::div);

	let chunk = chunk_as_global.resize_generic(Const::<C>, Const::<1>, 0);
	let block = block_as_global.resize_generic(Const::<B>, Const::<1>, 0);

	WorldCoordinate { chunk, block }
}

pub fn chunk_block_to_world<const W: usize, const C: usize, const B: usize>(
	extents: Vector<usize, B>,
	chunk: Vector<i32, C>,
	block: Vector<i32, B>,
) -> Vector<i32, W>
where
	Const<B>: DimMax<Const<W>, Output = Const<W>>,
	Const<C>: DimMax<Const<W>, Output = Const<W>>,
{
	let chunk_shape = extents.cast::<i32>();

	let chunk_shape_as_global = chunk_shape.resize_generic(Const::<W>, Const::<1>, 0);

	let chunk_as_global = chunk.resize_generic(Const::<W>, Const::<1>, 0);
	let block_as_global = block.resize_generic(Const::<W>, Const::<1>, 0);

	chunk_as_global + block_as_global.component_mul(&chunk_shape_as_global)
}

#[cfg(test)]
mod origin {
	use super::*;

	const X: usize = 5;
	const Y: usize = 7;
	const Z: usize = 1;
	const W: usize = 4;
	const V: usize = 10;

	const C: usize = X * Y * Z * W * V;

	fn helper(mut f: impl FnMut(usize, Vector<usize, 5>)) {
		let mut it = 0..C;

		for v in 0..V {
			for w in 0..W {
				for z in 0..Z {
					for y in 0..Y {
						for x in 0..X {
							f(it.next().unwrap(), Vector::from([x, y, z, w, v]))
						}
					}
				}
			}
		}
	}
	#[test]
	fn test_position_to_index() {
		helper(|expected, position| {
			let result =
				position_to_index(Vector::from([X, Y, Z, W, V]), position.cast()).unwrap();

			assert_eq!(expected as usize, result);
		})
	}
	#[test]
	fn test_index_to_position() {
		helper(|index, expected| {
			let result = index_to_position(Vector::from([X, Y, Z, W, V]), index).unwrap();

			assert_eq!(expected.cast(), result);
		})
	}
}

