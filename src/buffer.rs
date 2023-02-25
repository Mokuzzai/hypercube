use crate::lazy_unreachable;
use crate::math;
use crate::Chunk;
use crate::Cow;
use crate::Shape;

pub trait Storage: AsRef<[Self::Item]> + AsMut<[Self::Item]> {
	type Item;
}

pub trait FromFn: Storage {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self;
}

impl<T> Storage for [T] {
	type Item = T;
}

impl<T, const N: usize> Storage for [T; N] {
	type Item = T;
}

impl<T, const N: usize> FromFn for [T; N] {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self {
		assert_eq!(capacity, N);

		std::array::from_fn(f)
	}
}

impl<T> Storage for Vec<T> {
	type Item = T;
}

impl<T> FromFn for Vec<T> {
	fn from_fn(capacity: usize, mut f: impl FnMut(usize) -> Self::Item) -> Self {
		let mut buffer = Vec::with_capacity(capacity);

		for index in 0..capacity {
			buffer.push(f(index));
		}

		buffer
	}
}

impl<T> Storage for Box<[T]> {
	type Item = T;
}

impl<T> FromFn for Box<[T]> {
	fn from_fn(capacity: usize, f: impl FnMut(usize) -> Self::Item) -> Self {
		Vec::from_fn(capacity, f).into_boxed_slice()
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Buffer<T: ?Sized + Storage, S, const B: usize> {
	shape: S,
	buffer: T,
}

impl<T: ?Sized + Storage, S: Shape<B>, const B: usize> Chunk<B> for Buffer<T, S, B> {
	type Item = T::Item;
	type Shape = S;

	fn shape(&self) -> Cow<Self::Shape> {
		Cow::Borrowed(&self.shape)
	}
	fn as_slice(&self) -> &[Self::Item] {
		self.buffer.as_ref()
	}
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self.buffer.as_mut()
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Buffer<T, S, B> {
	/// # Panics
	/// This function panics if `S.capacity() != N`
	pub fn from_parts(shape: S, buffer: T) -> Self {
		assert_eq!(shape.capacity(), buffer.as_ref().len());

		Self { shape, buffer }
	}
}

impl<T: ?Sized + Storage, S: Shape<B>, const B: usize> Buffer<T, S, B> {
	pub fn iter(&self) -> impl Iterator<Item = &T::Item> {
		self.buffer.as_ref().iter()
	}
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T::Item> {
		self.buffer.as_mut().iter_mut()
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Buffer<T, S, B>
where
	T: FromFn,
{
	pub fn from_shape_index(shape: S, f: impl FnMut(usize) -> T::Item) -> Self {
		let capacity = shape.capacity();

		Self::from_parts(shape, T::from_fn(capacity, f))
	}
	pub fn from_shape_position(
		shape: S,
		mut f: impl FnMut(math::Point<i32, B>) -> T::Item,
	) -> Self {
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

impl<T: Storage, S: Shape<B>, const B: usize> Buffer<T, S, B>
where
	T: FromFn,
	S: Default,
{
	pub fn from_buffer(buffer: T) -> Self {
		Self::from_parts(S::default(), buffer)
	}
	pub fn from_index(f: impl FnMut(usize) -> T::Item) -> Self {
		Self::from_shape_index(S::default(), f)
	}
	pub fn from_position(f: impl FnMut(math::Point<i32, B>) -> T::Item) -> Self {
		Self::from_shape_position(S::default(), f)
	}
}

impl<T: Storage, S: Shape<B>, const B: usize> Default for Buffer<T, S, B>
where
	T: FromFn,
	T::Item: Default,
	S: Default,
{
	fn default() -> Self {
		Self::from_shape_default(S::default())
	}
}
