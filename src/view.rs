use crate::lazy_unreachable;
use crate::math;
use crate::math::Point;
use crate::shape::Shape;
use crate::storage::ContiguousMemory;
use crate::storage::ContiguousMemoryMut;
use crate::storage::FromFn;
use crate::storage::Storage;

#[derive(Debug, Eq, PartialEq)]
pub struct View<T: ?Sized, S, const B: usize> {
	shape: S,
	storage: T,
}

impl<T, S, const B: usize> View<T, S, B> {
	/// [`View`] may not behave correctly if `buffer.capacity() != shape.capacity()`
	pub fn new(buffer: T, shape: S) -> Self {
		Self {
			shape,
			storage: buffer,
		}
	}
	pub fn storage(&self) -> &T {
		&mut self.storage()
	}
	pub fn storage_mut(&mut self) -> &mut T {
		&mut self.storage()
	}
	pub fn shape(&self) -> &S {
		&self.shape
	}
	pub fn borrow(&self) -> ViewRef<T, &S, B> {
		ViewRef::new(&self.storage, &self.shape)
	}
	pub fn borrow_mut(&mut self) -> ViewMut<T, &S, B> {
		ViewMut::new(&mut self.storage, &self.shape)
	}
}

impl<T: ?Sized + ContiguousMemory, S: Shape<B>, const B: usize> View<T, S, B> {
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

impl<T: ?Sized + ContiguousMemoryMut, S: Shape<B>, const B: usize> View<T, S, B> {
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T::Item> {
		self.storage.as_mut_slice().iter_mut()
	}
	pub fn block_mut(&mut self, position: Point<i32, B>) -> Option<&mut T::Item> {
		let index = self.shape.position_to_index(position)?;

		self.storage.as_mut_slice().get_mut(index)
	}
	pub fn replace(&mut self, position: Point<i32, B>, mut block: T::Item) -> Result<T::Item, T::Item> {
		if let Some(slot) = self.block_mut(position) {
			Ok(std::mem::replace(slot, block))
		} else {
			Err(block)
		}
	}
}

impl<T: ContiguousMemory, S: Shape<B>, const B: usize> View<T, S, B>
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

impl<T: Storage, S: Shape<B>, const B: usize> View<T, S, B>
where
	T: FromFn,
	S: Default,
{
	pub fn from_buffer(buffer: T) -> Self {
		Self::new(buffer, S::default())
	}
	pub fn from_index(f: impl FnMut(usize) -> T::Item) -> Self {
		Self::from_shape_index(S::default(), f)
	}
	pub fn from_position(f: impl FnMut(math::Point<i32, B>) -> T::Item) -> Self {
		Self::from_shape_position(S::default(), f)
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Default for View<T, S, B>
where
	T: FromFn,
	T::Item: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}

pub type ViewRef<'a, T, S, const B: usize> = View<&'a T, S, B>;
pub type ViewMut<'a, T, S, const B: usize> = View<&'a mut T, S, B>;
