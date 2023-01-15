use crate::na;
use crate::DynamicMultiformShape;
use crate::Shape;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`Shape`]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Positions<const B: usize> {
	inner: Range<usize>,
	shape: DynamicMultiformShape<B>,
}

impl<const B: usize> Positions<B> {
	pub fn new(extents: na::Vector<usize, B>) -> Self {
		let shape = DynamicMultiformShape::new(extents);

		Self {
			inner: 0..shape.capacity(),
			shape,
		}
	}
}

impl<const B: usize> Iterator for Positions<B> {
	type Item = na::Vector<i32, B>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}
