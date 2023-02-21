use super::OrderedVector;
use crate::math;

use std::collections::btree_map;

pub struct OccupiedEntry<'a, T, const C: usize> {
	inner: btree_map::OccupiedEntry<'a, OrderedVector<C>, T>,
}

impl<'a, T, const C: usize> OccupiedEntry<'a, T, C> {
	pub fn position(&self) -> math::Position<C> {
		self.inner.key().coordinates
	}
	pub fn remove_entry(self) -> (math::Position<C>, T) {
		let (position, chunk) = self.inner.remove_entry();

		(position.coordinates, chunk)
	}
	pub fn chunk(&self) -> &T {
		self.inner.get()
	}
	pub fn chunk_mut(&mut self) -> &mut T {
		self.inner.get_mut()
	}
	pub fn into_mut_chunk(self) -> &'a mut T {
		self.inner.into_mut()
	}
	pub fn insert(&mut self, value: T) -> T {
		self.inner.insert(value)
	}
	pub fn remove(self) -> T {
		self.inner.remove()
	}
}

pub struct VacantEntry<'a, T, const C: usize> {
	inner: btree_map::VacantEntry<'a, OrderedVector<C>, T>,
}

impl<'a, T, const C: usize> VacantEntry<'a, T, C> {
	pub fn position(&self) -> math::Position<C> {
		self.inner.key().coordinates
	}
	pub fn insert(self, value: T) -> &'a mut T {
		self.inner.insert(value)
	}
	pub fn into_key(self) -> math::Position<C> {
		self.inner.into_key().coordinates
	}
}
const _: () = {
	use std::fmt::*;

	impl<'a, T: Debug, const C: usize> Debug for OccupiedEntry<'a, T, C> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.inner, f)
		}
	}

	impl<'a, T: Debug, const C: usize> Debug for VacantEntry<'a, T, C> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.inner, f)
		}
	}
};

#[derive(Debug)]
pub enum Entry<'a, T, const C: usize> {
	Vacant(VacantEntry<'a, T, C>),
	Occupied(OccupiedEntry<'a, T, C>),
}

impl<'a, T, const C: usize> Entry<'a, T, C> {
	pub(crate) fn from(entry: btree_map::Entry<'a, OrderedVector<C>, T>) -> Self {
		match entry {
			btree_map::Entry::Vacant(inner) => Self::Vacant(VacantEntry { inner }),
			btree_map::Entry::Occupied(inner) => Self::Occupied(OccupiedEntry { inner }),
		}
	}
	pub fn and_modify<F>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut T),
	{
		if let Self::Occupied(ref mut chunk) = self {
			f(chunk.chunk_mut())
		}

		self
	}
	pub fn or_default(self) -> &'a mut T
	where
		T: Default,
	{
		match self {
			Self::Occupied(entry) => entry.into_mut_chunk(),
			Self::Vacant(entry) => entry.insert(Default::default()),
		}
	}
	pub fn chunk_mut(&mut self) -> Option<&mut T> {
		let Self::Occupied(entry) = self else { return None };

		Some(entry.chunk_mut())
	}
	pub fn position(&self) -> math::Position<C> {
		match *self {
			Self::Occupied(ref entry) => entry.position(),
			Self::Vacant(ref entry) => entry.position(),
		}
	}
	pub fn or_insert(self, default: T) -> &'a mut T {
		match self {
			Self::Occupied(entry) => entry.into_mut_chunk(),
			Self::Vacant(entry) => entry.insert(default),
		}
	}
	pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
		match self {
			Self::Occupied(entry) => entry.into_mut_chunk(),
			Self::Vacant(entry) => entry.insert(default()),
		}
	}
	pub fn or_insert_with_key<F: FnOnce(math::Position<C>) -> T>(
		self,
		default: F,
	) -> &'a mut T {
		match self {
			Self::Occupied(entry) => entry.into_mut_chunk(),
			Self::Vacant(entry) => {
				let value = default(entry.position());
				entry.insert(value)
			}
		}
	}
}
