use crate::na;
use crate::SVector;
use crate::Shape;

use std::cmp::Ordering;

pub struct OrderedVector<S: Shape>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	pub coordinates: SVector<S>,
}

impl<S: Shape> OrderedVector<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	pub fn new(coordinates: SVector<S>) -> Self {
		Self { coordinates }
	}
}

impl<S: Shape> PartialEq for OrderedVector<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	fn eq(&self, other: &Self) -> bool {
		self.coordinates.eq(&other.coordinates)
	}
}

impl<S: Shape> Eq for OrderedVector<S> where na::DefaultAllocator: na::Allocator<i32, S::Dim> {}

impl<S: Shape> PartialOrd for OrderedVector<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.try_fold(Ordering::Equal, |acc, (a, b)| {
				Some(acc.then(a.partial_cmp(b)?))
			})
	}
}

impl<S: Shape> Ord for OrderedVector<S>
where
	na::DefaultAllocator: na::Allocator<i32, S::Dim>,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp(b)))
	}
}

const _: () = {
	use std::fmt::*;

	impl<S: Shape> Debug for OrderedVector<S>
	where
		na::DefaultAllocator: na::Allocator<i32, S::Dim>,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.coordinates, f)
		}
	}
};
