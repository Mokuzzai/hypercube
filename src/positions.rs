use crate::na;
use crate::IndexableShape;
use crate::SVector;

use std::ops::Range;

/// [`Iterator`] over all the possible positions of a [`IndexableShape`]
pub struct Positions<S: IndexableShape>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	inner: Range<usize>,
	shape: S,
}

impl<S: IndexableShape> Positions<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	pub fn new(shape: S) -> Self {
		Self {
			inner: 0..shape.capacity(),
			shape,
		}
	}
}

impl<S: IndexableShape> Iterator for Positions<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	type Item = SVector<S>;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.inner.next()?;

		self.shape.index_to_position(next)
	}
}

const _: () = {
	use std::fmt::*;

	impl<S: IndexableShape> Debug for Positions<S>
	where
		na::DefaultAllocator: na::Allocator<i32, S::Dim>,
		S: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("Positions")
				.field("inner", &self.inner)
				.field("shape", &self.shape)
				.finish()
		}
	}
};
