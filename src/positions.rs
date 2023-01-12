use crate::na;
use crate::Shape;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`Shape`]

#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
pub struct Positions<S: Shape<D>, const D: usize> {
	inner: Range<usize>,
	shape: S,
}

impl<S: Shape<D>, const D: usize> Positions<S, D> {
	pub fn new(shape: S) -> Self {
		Self {
			inner: 0..shape.capacity(),
			shape,
		}
	}
}

impl<S: Shape<D>, const D: usize> Iterator for Positions<S, D>
{
	type Item = na::Vector<i32, D>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}

const _: () = {
	use std::fmt::*;

	impl<S: Shape<D>, const D: usize> Positions<S, D>
		where S: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Positions")
				.field("inner", &self.inner)
				.field("shape", &self.shape)
				.finish()
		}
	}
};
