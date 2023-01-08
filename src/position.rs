use crate::utils::Range;

use std::cmp::Ordering;

use nalgebra::OVector as Vector;
use nalgebra::Point;

pub trait Coordinate: nalgebra::Scalar + num::Num + simba::scalar::SubsetOf<usize> {}

impl<T: nalgebra::Scalar + num::Num + simba::scalar::SubsetOf<usize>> Coordinate for T {}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BlockPosition<
	Scc: Coordinate,
	Cc: Coordinate,
	const CHUNK_DIMENSIONS: usize,
	const WORLD_DIMENSIONS: usize,
> {
	pub sub_chunk: SubChunkPosition<Scc, CHUNK_DIMENSIONS>,
	pub chunk: ChunkPosition<Cc, WORLD_DIMENSIONS>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ChunkPosition<T: Coordinate, const D: usize> {
	pub coordinates: Point<T, D>,
}

impl<T: Coordinate + PartialOrd, const D: usize> PartialOrd for ChunkPosition<T, D> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.try_fold(Ordering::Equal, |acc, (a, b)| {
				Some(acc.then(a.partial_cmp(b)?))
			})
	}
}

impl<T: Coordinate + Ord, const D: usize> Ord for ChunkPosition<T, D> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp(b)))
	}
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct SubChunkPosition<T: Coordinate, const D: usize> {
	pub coordinates: Point<T, D>,
}

impl<T: Coordinate, const D: usize> SubChunkPosition<T, D> {
	pub fn to_index(self, stride: usize) -> Option<usize> {
		position_to_index(stride, self.coordinates.coords.cast().into())
	}
	pub fn from_index(stride: usize, index: usize) -> Option<Self> {
		index_to_position(stride, index)
			.map(|array| {
				Some(Self {
					coordinates: Point::from(
						Vector::<usize, nalgebra::base::dimension::Const<D>>::from(array)
							.try_cast::<T>()?,
					),
				})
			})
			.flatten()
	}
}

pub fn position_to_index<const N: usize>(stride: usize, position: [usize; N]) -> Option<usize> {
	position
		.into_iter()
		.enumerate()
		.try_fold(0, |acc, (exp, coordinate)| {
			if coordinate < stride {
				Some(acc + coordinate * stride.pow(exp as u32))
			} else {
				None
			}
		})
}

pub fn index_to_position<const N: usize>(stride: usize, index: usize) -> Option<[usize; N]> {
	if index >= stride.pow(N as u32) {
		return None;
	}

	let mut prev = 0;

	Some(std::array::from_fn(|exp| {
		let this = ((index - prev) / stride.pow(exp as u32)) % stride;

		prev = this;

		this
	}))
}

#[cfg(test)]
mod tests {
	use super::*;

	const S: usize = 16;
	const C: usize = 16 * 16 * 16;

	#[test]
	fn test_from_indices() {
		let mut it = 0..C;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = it.next().unwrap();
					let result = position_to_index(S, [x, y, z]).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}

	#[test]
	fn test_to_indices() {
		let mut it = 0..C;

		for z in 0..S {
			for y in 0..S {
				for x in 0..S {
					let expected = [x, y, z];
					let result = index_to_position(S, it.next().unwrap()).unwrap();

					assert_eq!(expected, result);
				}
			}
		}
	}
}

pub struct Positions<U: Coordinate, const S: usize, const D: usize, const C: usize> {
	inner: Range<C>,
	_coord: std::marker::PhantomData<U>,
}

impl<U: Coordinate, const S: usize, const D: usize, const C: usize> Positions<U, D, S, C> {
	pub fn new() -> Self {
		Self {
			inner: Range::new(0),
			_coord: std::marker::PhantomData,
		}
	}
}

impl<U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for Positions<U, S, D, C>
{
	type Item = SubChunkPosition<U, D>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		SubChunkPosition::from_index(S, next)
	}
}
