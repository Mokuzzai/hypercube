use crate::lazy_unreachable;
use crate::math;
use crate::math::Point;
use crate::shape::Cow;
use crate::shape::Shape;
use crate::storage::ContiguousMemory;
use crate::storage::ContiguousMemoryMut;
use crate::storage::FromFn;
use crate::storage::Storage;

#[derive(Debug, Eq, PartialEq)]
pub struct View<'a, T: ?Sized, S, const B: usize> {
	shape: Cow<'a, S>,
	storage: T,
}

impl<'a, T, S, const B: usize> View<'a, T, S, B> {
	/// [`View`] may not behave correctly if `buffer.capacity() != shape.capacity()`
	pub fn new(buffer: T, shape: impl Into<Cow<'a, S>>) -> Self {
		Self {
			shape: shape.into(),
			storage: buffer,
		}
	}
	// NOTE: `'a` is needed for self because `self` might own its shape
	pub fn borrow(&'a self) -> ViewRef<'a, T, Cow<'a, S>, B> {
		ViewRef::new(&self.storage, &self.shape)
	}
	pub fn borrow_mut(&'a mut self) -> ViewMut<'a, T, Cow<'a, S>, B> {
		ViewMut::new(&mut self.storage, &self.shape)
	}
}

impl<'a, T: ?Sized + ContiguousMemory, S: Shape<B>, const B: usize> View<'a, T, S, B> {
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
	pub fn block(&'a self, position: Point<i32, B>) -> Option<&'a T::Item> {
		let index = self.shape.position_to_index(position)?;

		self.storage.as_slice().get(index)
	}
}

impl<'a, T: ?Sized + ContiguousMemoryMut, S: Shape<B>, const B: usize> View<'a, T, S, B> {
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T::Item> {
		self.storage.as_mut_slice().iter_mut()
	}
	pub fn block_mut(&'a mut self, position: Point<i32, B>) -> Option<&'a mut T::Item> {
		let index = self.shape.position_to_index(position)?;

		self.storage.as_mut_slice().get_mut(index)
	}
}

impl<'a, T: ContiguousMemory, S: Shape<B>, const B: usize> View<'a, T, S, B>
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

impl<'a, T: Storage, S: Shape<B>, const B: usize> View<'a, T, S, B>
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

impl<'a, T: Storage, S: Shape<B>, const B: usize> Default for View<'a, T, S, B>
where
	T: FromFn,
	T::Item: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}

pub type ViewRef<'a, T, S, const B: usize> = View<'a, &'a T, S, B>;
pub type ViewMut<'a, T, S, const B: usize> = View<'a, &'a mut T, S, B>;
