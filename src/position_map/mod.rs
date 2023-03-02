mod ordered_point;

pub mod entry;

pub use entry::Entry;
pub use entry::OccupiedEntry;
pub use entry::VacantEntry;
pub use ordered_point::OrderedPoint;

use crate::math::Point;

use std::collections::BTreeMap;

pub struct PositionMap<T, const D: usize> {
	inner: BTreeMap<OrderedPoint<D>, T>,
}

impl<T, const D: usize> PositionMap<T, D> {
	pub fn new() -> Self {
		Self {
			inner: BTreeMap::new(),
		}
	}
	pub fn len(&self) -> usize {
		self.inner.len()
	}
	pub fn positions(&self) -> impl '_ + Iterator<Item = Point<i32, D>> {
		self.inner.keys().map(|a| a.coordinates)
	}
	pub fn iter(&self) -> impl Iterator<Item = (Point<i32, D>, &T)> {
		self.inner.iter().map(|(a, b)| (a.coordinates, b))
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = (Point<i32, D>, &mut T)> {
		self.inner.iter_mut().map(|(a, b)| (a.coordinates, b))
	}
}

/// # `Chunk` manipulation
impl<T, const D: usize> PositionMap<T, D> {
	pub fn get(&self, position: Point<i32, D>) -> Option<&T> {
		self.inner.get(&OrderedPoint::new(position))
	}
	pub fn get_mut(&mut self, position: Point<i32, D>) -> Option<&mut T> {
		self.inner.get_mut(&OrderedPoint::new(position))
	}
	pub fn remove(&mut self, position: Point<i32, D>) -> Option<T> {
		self.inner.remove(&OrderedPoint::new(position))
	}
	pub fn insert(&mut self, position: Point<i32, D>, chunk: T) -> Option<T> {
		self.inner.insert(OrderedPoint::new(position), chunk)
	}
	pub fn entry(&mut self, position: Point<i32, D>) -> entry::Entry<T, D> {
		let entry = self.inner.entry(OrderedPoint::new(position));

		entry::Entry::from(entry)
	}
	pub fn values(&self) -> impl Iterator<Item = &T> {
		self.inner.values()
	}
	pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
		self.inner.values_mut()
	}
}

impl<T, const D: usize> Default for PositionMap<T, D> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const D: usize> Eq for PositionMap<T, D> where T: Eq {}

impl<T, const D: usize> PartialEq for PositionMap<T, D>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl<T, const D: usize> Clone for PositionMap<T, D>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
		}
	}
}

const _: () = {
	use std::fmt::*;

	impl<T, const D: usize> Debug for PositionMap<T, D>
	where
		T: Debug,
	{
		fn fmt(&self, f: &mut Formatter) -> Result {
			f.debug_struct("PositionMap")
				.field("inner", &self.inner)
				.finish()
		}
	}
};
