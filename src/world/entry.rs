use super::OrderedVector;
use crate::math;

use std::collections::btree_map;

pub struct OccupiedEntry<'a, T, const B: usize> {
	inner: btree_map::OccupiedEntry<'a, OrderedVector<B>, T>,
}

impl<'a, T, const B: usize> OccupiedEntry<'a, T, B> {
	pub fn position(&self) -> &math::Vector<i32, B> {
		&self.inner.key().coordinates
	}
	pub fn remove_entry(self) -> (math::Vector<i32, B>, T) {
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

pub struct VacantEntry<'a, T, const B: usize> {
	inner: btree_map::VacantEntry<'a, OrderedVector<B>, T>,
}

impl<'a, T, const B: usize> VacantEntry<'a, T, B> {
	pub fn position(&self) -> &math::Vector<i32, B> {
		&self.inner.key().coordinates
	}
	pub fn insert(self, value: T) -> &'a mut T {
		self.inner.insert(value)
	}
	pub fn into_key(self) -> math::Vector<i32, B> {
		self.inner.into_key().coordinates
	}
}
const _: () = {
	use std::fmt::*;

	impl<'a, T: Debug, const B: usize> Debug for OccupiedEntry<'a, T, B> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.inner, f)
		}
	}

	impl<'a, T: Debug, const B: usize> Debug for VacantEntry<'a, T, B> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			Debug::fmt(&self.inner, f)
		}
	}
};

#[derive(Debug)]
pub enum Entry<'a, T, const B: usize> {
	Vacant(VacantEntry<'a, T, B>),
	Occupied(OccupiedEntry<'a, T, B>),
}

impl<'a, T, const B: usize> Entry<'a, T, B> {
	pub(crate) fn from(entry: btree_map::Entry<'a, OrderedVector<B>, T>) -> Self {
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
	pub fn position(&self) -> &math::Vector<i32, B> {
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
	pub fn or_insert_with_key<F: FnOnce(&math::Vector<i32, B>) -> T>(
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
