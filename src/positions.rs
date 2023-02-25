use crate::rt::Multiform;
use crate::math;
use crate::Shape;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`Shape`]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Positions<const B: usize> {
	inner: Range<usize>,
	shape: Multiform<B>,
}

impl<const B: usize> Positions<B> {
	pub fn new(extents: math::Vector<usize, B>) -> Self {
		let shape = Multiform::new(extents);

		Self {
			inner: 0..shape.capacity(),
			shape,
		}
	}
	pub fn extents(&self) -> math::Vector<usize, B> {
		self.shape.extents()
	}
}

impl<const B: usize> Iterator for Positions<B> {
	type Item = math::Point<i32, B>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OffsetPositions<const B: usize> {
	inner: Range<usize>,
	offset: math::Vector<i32, B>,
	shape: Multiform<B>,
}

impl<const B: usize> Default for OffsetPositions<B> {
	fn default() -> Self {
		Self {
			inner: Range::default(),
			offset: math::Vector::from([0; B]),
			shape: Multiform::default(),
		}
	}
}

impl<const B: usize> OffsetPositions<B> {
	pub fn new(extents: math::Vector<usize, B>, offset: math::Vector<i32, B>) -> Self {
		let shape = Multiform::new(extents);

		Self {
			inner: 0..shape.capacity(),
			offset,
			shape,
		}
	}
	pub fn extents(&self) -> math::Vector<usize, B> {
		self.shape.extents()
	}
	pub fn offset(&self) -> math::Vector<i32, B> {
		self.offset
	}
}

impl<const B: usize> Iterator for OffsetPositions<B> {
	type Item = math::Point<i32, B>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		math::position_index_conversion::index_to_position_offset(
			self.shape.extents(),
			self.offset,
			next,
		)
	}
}
