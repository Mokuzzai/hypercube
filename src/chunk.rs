use crate::lazy_unreachable;
use crate::math;
use crate::math::Point;
use crate::shape::Shape;
use crate::storage::ContiguousMemory;
use crate::storage::ContiguousMemoryMut;
use crate::storage::ReadStorage;
use crate::storage::FromFn;
use crate::storage::Storage;

use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Chunk<T: ?Sized, S, const B: usize> {
	pub shape: S,
	pub storage: T,
}

impl<T, S, const B: usize> Chunk<T, S, B> {
	/// [`Chunk`] may not behave correctly if `buffer.capacity() != shape.capacity()`
	pub const fn new(buffer: T, shape: S) -> Self {
		Self {
			shape,
			storage: buffer,
		}
	}
	pub fn into_raw_parts(self) -> (S, T) {
		(self.shape, self.storage)
	}
	pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Chunk<U, S, B> {
		Chunk::new(f(self.storage), self.shape)
	}
}

impl<T: ?Sized, S, const B: usize> Chunk<T, S, B> {
	pub fn storage(&self) -> &T {
		&self.storage
	}
	pub fn storage_mut(&mut self) -> &mut T {
		&mut self.storage
	}
	pub fn shape(&self) -> &S {
		&self.shape
	}
}

impl<T: ?Sized, S: Copy, const B: usize> Chunk<T, S, B> {
	pub fn as_ref(&self) -> ChunkRef<T, S, B> {
		ChunkRef::new(&self.storage, self.shape)
	}
	pub fn as_mut(&mut self) -> ChunkMut<T, S, B> {
		ChunkMut::new(&mut self.storage, self.shape)
	}
}

impl<'a, T: Clone, S, const B: usize> ChunkRef<'a, T, S, B> {
	pub fn cloned(self) -> Chunk<T, S, B> {
		self.map(Clone::clone)
	}
}

impl<T: ?Sized + Deref, S: Copy, const B: usize> Chunk<T, S, B> {
	pub fn as_deref(&self) -> ChunkRef<T::Target, S, B> {
		ChunkRef::new(&*self.storage, self.shape)
	}
}

impl<T: ?Sized + DerefMut, S: Copy, const B: usize> Chunk<T, S, B> {
	pub fn as_deref_mut(&mut self) -> ChunkMut<T::Target, S, B> {
		ChunkMut::new(&mut *self.storage, self.shape)
	}
}

impl<T: ?Sized + ReadStorage<usize>, S, const B: usize> Chunk<T, S, B> {
	pub fn read(&self, index: usize) -> Option<T::Item> {
		self.storage().read(index)
	}
}

impl<T: ?Sized + ReadStorage<usize>, S: Shape<B>, const B: usize> Chunk<T, S, B> {
	pub fn read_position(&self, position: Point<i32, B>) -> Option<T::Item> {
		let index = self.shape().position_to_index(position)?;

		self.read(index)
	}
}

impl<T: ?Sized + ContiguousMemory, S: Shape<B>, const B: usize> Chunk<T, S, B> {
	pub fn iter(&self) -> impl Iterator<Item = &T::Item> {
		self.storage.as_slice().iter()
	}
	pub fn block_positions(&self) -> impl Iterator<Item = (Point<i32, B>, &T::Item)> {
		self.iter().enumerate().map(|(index, block)| {
			(
				self.shape
					.index_to_position(index)
					.unwrap_or_else(lazy_unreachable!()),
				block,
			)
		})
	}
	pub fn block(&self, position: Point<i32, B>) -> Option<&T::Item> {
		let index = self.shape.position_to_index(position)?;

		self.storage.as_slice().get(index)
	}
}

impl<T: ?Sized + ContiguousMemoryMut, S: Shape<B>, const B: usize> Chunk<T, S, B> {
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T::Item> {
		self.storage.as_mut_slice().iter_mut()
	}
	pub fn block_positions_mut(&mut self) -> impl Iterator<Item = (Point<i32, B>, &mut T::Item)> {
		self.storage.as_mut_slice().iter_mut().enumerate().map(|(index, block)| {
			(
				self.shape
					.index_to_position(index)
					.unwrap_or_else(lazy_unreachable!()),
				block,
			)
		})
	}
	pub fn block_mut(&mut self, position: Point<i32, B>) -> Option<&mut T::Item> {
		let index = self.shape.position_to_index(position)?;

		self.storage.as_mut_slice().get_mut(index)
	}
	pub fn replace(&mut self, position: Point<i32, B>, block: T::Item) -> Result<T::Item, T::Item> {
		if let Some(slot) = self.block_mut(position) {
			Ok(std::mem::replace(slot, block))
		} else {
			Err(block)
		}
	}
}

impl<T, S: Shape<B>, const B: usize> Chunk<T, S, B>
where
	T: FromFn,
{
	pub fn from_shape_index(shape: S, f: impl FnMut(usize) -> T::Item) -> Self {
		let capacity = shape.capacity();

		Self::new(T::from_fn(capacity, f), shape)
	}
	pub fn from_shape_position(shape: S, mut f: impl FnMut(Point<i32, B>) -> T::Item) -> Self {
		let extents = shape.extents();

		Self::from_shape_index(shape, |index| {
			f(math::index_to_position(extents, index).unwrap_or_else(lazy_unreachable!()))
		})
	}
	pub fn from_shape_default(shape: S) -> Self
	where
		T::Item: Default,
	{
		Self::from_shape_index(shape, |_| T::Item::default())
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Chunk<T, S, B>
where
	S: Default,
{
	pub fn from_storage(storage: T) -> Self {
		Self::new(storage, S::default())
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Chunk<T, S, B>
where
	T: FromFn,
	S: Default,
{
	pub fn from_index(f: impl FnMut(usize) -> T::Item) -> Self {
		Self::from_shape_index(S::default(), f)
	}
	pub fn from_position(f: impl FnMut(math::Point<i32, B>) -> T::Item) -> Self {
		Self::from_shape_position(S::default(), f)
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Default for Chunk<T, S, B>
where
	T: FromFn,
	T::Item: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}

pub type ChunkRef<'a, T, S, const B: usize> = Chunk<&'a T, S, B>;
pub type ChunkMut<'a, T, S, const B: usize> = Chunk<&'a mut T, S, B>;
