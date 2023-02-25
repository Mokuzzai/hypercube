use crate::math;

use std::cmp::Ordering;

#[derive(Clone)]
#[repr(transparent)]
pub struct OrderedPoint<const D: usize> {
	pub coordinates: math::Point<i32, D>,
}

impl<const D: usize> OrderedPoint<D> {
	pub fn new(coordinates: math::Point<i32, D>) -> Self {
		Self { coordinates }
	}
}

impl<const D: usize> PartialEq for OrderedPoint<D> {
	fn eq(&self, other: &Self) -> bool {
		self.coordinates.eq(&other.coordinates)
	}
}

impl<const D: usize> Eq for OrderedPoint<D> {}

impl<const D: usize> PartialOrd for OrderedPoint<D> {
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

impl<const D: usize> Ord for OrderedPoint<D> {
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

	impl<const D: usize> Debug for OrderedPoint<D> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.coordinates, f)
		}
	}
};
