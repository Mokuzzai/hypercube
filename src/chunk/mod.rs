mod imp;

pub use imp::chunk16d2::Chunk16d2;
pub use imp::chunk16d3::Chunk16d3;
pub use imp::chunk32d2::Chunk32d2;
pub use imp::chunk32d3::Chunk32d3;

use crate::Positions;

use crate::Coordinate;
use crate::SubChunkPosition;

pub trait Chunk<const S: usize, const D: usize, const C: usize> {
	type Item;

	type Coordinate: Coordinate;

	fn stride(&self) -> usize {
		S
	}

	fn dimensions(&self) -> usize {
		D
	}

	fn capacity(&self) -> usize {
		C
	}

	fn array(&self) -> &[Self::Item; C];
	fn array_mut(&mut self) -> &mut [Self::Item; C];

	fn get(&mut self, position: SubChunkPosition<Self::Coordinate, D>) -> Option<&Self::Item> {
		self.array().get(position.to_index(S)?)
	}

	fn get_mut(
		&mut self,
		position: SubChunkPosition<Self::Coordinate, D>,
	) -> Option<&mut Self::Item> {
		self.array_mut().get_mut(position.to_index(S)?)
	}

	fn into_iter(self) -> IntoIter<Self::Item, Self::Coordinate, S, D, C>
	where
		Self: Sized,
	{
		unimplemented!()
	}

	fn iter(&self) -> Iter<Self::Item, Self::Coordinate, S, D, C> {
		Iter {
			values: self.array().iter(),
			positions: Positions::new(),
		}
	}

	fn iter_mut(&mut self) -> IterMut<Self::Item, Self::Coordinate, S, D, C> {
		IterMut {
			values: self.array_mut().iter_mut(),
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
