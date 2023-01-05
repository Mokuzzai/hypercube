mod imp;

pub use imp::chunk16d2::Chunk16d2;
pub use imp::chunk16d3::Chunk16d3;
pub use imp::chunk32d2::Chunk32d2;
pub use imp::chunk32d3::Chunk32d3;

use crate::utils::index_to_position;
use crate::utils::position_to_index;

use crate::utils::Range;

pub trait Chunk<const S: usize, const D: usize, const C: usize> {
	type Item;

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

	fn get(&mut self, position: [usize; D]) -> Option<&Self::Item> {
		self.array()
			.get(position_to_index(self.stride(), position)?)
	}

	fn get_mut(&mut self, position: [usize; D]) -> Option<&mut Self::Item> {
		let stride = self.stride();

		self.array_mut()
			.get_mut(position_to_index(stride, position)?)
	}

	fn into_iter(self) -> IntoIter<Self::Item, S, D, C>
	where
		Self: Sized,
	{
		unimplemented!()
	}

	fn iter(&self) -> Iter<Self::Item, S, D, C> {
		Iter {
			values: self.array().iter(),
			positions: Positions::new(),
		}
	}

	fn iter_mut(&mut self) -> IterMut<Self::Item, S, D, C> {
		IterMut {
			values: self.array_mut().iter_mut(),
			positions: Positions::new(),
		}
	}
}

pub struct IntoIter<T, const S: usize, const D: usize, const C: usize> {
	values: std::array::IntoIter<T, C>,
	positions: Positions<S, D, C>,
}

impl<T, const S: usize, const D: usize, const C: usize> Iterator for IntoIter<T, S, D, C> {
	type Item = ([usize; D], T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct Iter<'a, T, const S: usize, const D: usize, const C: usize> {
	values: std::slice::Iter<'a, T>,
	positions: Positions<S, D, C>,
}

impl<'a, T, const S: usize, const D: usize, const C: usize> Iterator for Iter<'a, T, S, D, C> {
	type Item = ([usize; D], &'a T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct IterMut<'a, T, const S: usize, const D: usize, const C: usize> {
	values: std::slice::IterMut<'a, T>,
	positions: Positions<S, D, C>,
}

impl<'a, T, const S: usize, const D: usize, const C: usize> Iterator for IterMut<'a, T, S, D, C> {
	type Item = ([usize; D], &'a mut T);

	fn next(&mut self) -> Option<Self::Item> {
		self.positions.by_ref().zip(self.values.by_ref()).next()
	}
}

pub struct Positions<const S: usize, const D: usize, const C: usize> {
	inner: Range<C>,
}

impl<const S: usize, const D: usize, const C: usize> Positions<D, S, C> {
	fn new() -> Self {
		Self {
			inner: Range::new(0),
		}
	}
}

impl<const S: usize, const D: usize, const C: usize> Iterator for Positions<S, D, C> {
	type Item = [usize; D];

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		index_to_position(S, next)
	}
}
