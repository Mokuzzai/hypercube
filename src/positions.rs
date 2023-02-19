use crate::dynamic::Multiform;
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
}

impl<const B: usize> Iterator for Positions<B> {
	type Item = math::Vector<i32, B>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}
