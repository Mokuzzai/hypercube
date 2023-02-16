use crate::na;

use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone)]
pub struct OrderedVector<const C: usize> {
	pub coordinates: na::Vector<i32, C>,
}

impl<const C: usize> OrderedVector<C> {
	pub fn new(coordinates: na::Vector<i32, C>) -> Self {
		Self { coordinates }
	}
}

impl<const C: usize> Hash for OrderedVector<C> {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.coordinates.iter().for_each(|extent| extent.hash(state))
	}
}

impl<const C: usize> PartialEq for OrderedVector<C> {
	fn eq(&self, other: &Self) -> bool {
		self.coordinates.eq(&other.coordinates)
	}
}

impl<const C: usize> Eq for OrderedVector<C> {}

impl<const C: usize> PartialOrd for OrderedVector<C> {
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

impl<const C: usize> Ord for OrderedVector<C> {
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

	impl<const C: usize> Debug for OrderedVector<C> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.coordinates, f)
		}
	}
};
