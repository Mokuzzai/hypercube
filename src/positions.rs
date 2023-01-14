use crate::na;
use crate::DynamicMultiformShape;
use crate::Shape;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`Shape`]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Positions<const D: usize> {
	inner: Range<usize>,
	shape: DynamicMultiformShape<D>,
}

impl<const D: usize> Positions<D> {
	pub fn new<S: Shape<D>>(shape: &S) -> Self {
		Self {
			inner: 0..shape.capacity(),
			shape: DynamicMultiformShape::new(shape.extents()),
		}
	}
}

impl<const D: usize> Iterator for Positions<D> {
	type Item = na::Vector<i32, D>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}
