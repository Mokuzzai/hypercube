use crate::na;
use crate::Shape;

use std::cmp::Ordering;

pub struct OrderedVector<const D: usize> {
	pub coordinates: na::Vector<i32, D>,
}

impl<const D: usize> OrderedVector<D> {
	pub fn new(coordinates: na::Vector<i32, D>) -> Self {
		Self { coordinates }
	}
}

impl<const D: usize> PartialEq for OrderedVector<D>
{
	fn eq(&self, other: &Self) -> bool {
		self.coordinates.eq(&other.coordinates)
	}
}

impl<const D: usize> Eq for OrderedVector<D> {}

impl<const D: usize> PartialOrd for OrderedVector<D> {
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

impl<const D: usize> Ord for OrderedVector<D> {
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

	impl<const D: usize> Debug for OrderedVector<D> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.coordinates, f)
		}
	}
};
