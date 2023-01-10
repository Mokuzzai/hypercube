use crate::IndexableShape;
use crate::SVector;
use crate::na;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`IndexableShape`]
pub struct Positions<S: IndexableShape> {
	inner: Range<usize>,
	shape: S,
}

impl<S: IndexableShape> Positions<S> {
	pub fn new(shape: S) -> Self {
		Self {
			inner: 0..shape.capacity(),
			shape,
		}
	}
}

impl<S: IndexableShape> Iterator for Positions<S>
where
	na::DefaultAllocator: na::Allocator<S::Coordinate, S::Dimension>,
{
	type Item = SVector<S>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}
