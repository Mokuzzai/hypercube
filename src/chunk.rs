

pub type Chunk16d2<T> = Chunk<T, u8, 16, 2, { 16 * 16 }>;
pub type Chunk16d3<T> = Chunk<T, u8, 16, 3, { 16 * 16 * 16 }>;
pub type Chunk32d2<T> = Chunk<T, u8, 32, 2, { 32 * 32 }>;
pub type Chunk32d3<T> = Chunk<T, u8, 32, 3, { 32 * 32 * 32}>;

use crate::Positions;

use crate::Coordinate;
use crate::SubChunkPosition;

use std::marker::PhantomData;

pub struct Chunk<T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	inner: [T; C],
	_coord: PhantomData<U>,
}


impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize> Chunk<T, U, S, D, C> {
	pub const STRIDE: usize = S;
	pub const DIMENSIONS: usize = D;
	pub const CAPACITY: usize = C;

	pub fn stride(&self) -> usize {
		S
	}

	pub fn dimensions(&self) -> usize {
		D
	}

	pub fn capacity(&self) -> usize {
		C
	}

	pub fn get(&mut self, position: SubChunkPosition<U, D>) -> Option<&T> {
		self.inner.get(position.to_index(S)?)
	}

	pub fn get_mut(
		&mut self,
		position: SubChunkPosition<U, D>,
	) -> Option<&mut T> {
		self.inner.get_mut(position.to_index(S)?)
	}

	pub fn into_iter(self) -> IntoIter<T, U, S, D, C> {
		IntoIter {
			values: self.inner.into_iter(),
			positions: Positions::new(),
		}
	}

	pub fn iter(&self) -> Iter<T, U, S, D, C> {
		Iter {
			values: self.inner.iter(),
			positions: Positions::new(),
		}
	}

	pub fn iter_mut(&mut self) -> IterMut<T, U, S, D, C> {
		IterMut {
			values: self.inner.iter_mut(),
			positions: Positions::new(),
		}
	}
}

pub struct IntoIter<T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::array::IntoIter<T, C>,
	positions: Positions<U, S, D, C>,
}

impl<T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for IntoIter<T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct Iter<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::slice::Iter<'a, T>,
	positions: Positions<U, S, D, C>,
}

impl<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for Iter<'a, T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct IterMut<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> {
	values: std::slice::IterMut<'a, T>,
	positions: Positions<U, S, D, C>,
}

impl<'a, T, U: Coordinate, const S: usize, const D: usize, const C: usize> Iterator
	for IterMut<'a, T, U, S, D, C>
{
	type Item = (SubChunkPosition<U, D>, &'a mut T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}
