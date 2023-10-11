use super::math;
use super::ChunkMut;
use crate::position_map;
use crate::storage::FromFn;
use crate::Shape;

#[derive(Debug)]
pub struct Entry<'a, T, S, const C: usize, const B: usize> {
	inner: position_map::Entry<'a, T, C>,
	shape: S,
}

impl<'a, T, S: Shape<B>, const C: usize, const B: usize> Entry<'a, T, S, C, B> {
	pub(crate) fn from(inner: position_map::Entry<'a, T, C>, shape: S) -> Self {
		Self { inner, shape }
	}

	pub fn and_modify<F>(self, f: F) -> Self
	where
		F: FnOnce(ChunkMut<T, S, B>),
	{
		let Self { inner, shape } = self;

		Self::from(
			inner.and_modify(|storage| f(ChunkMut::new(storage, shape))),
			shape,
		)
	}
	pub fn or_default(self) -> ChunkMut<'a, T, S, B>
	where
		T: FromFn,
		T::Item: Default,
	{
		let Self { inner, shape } = self;

		ChunkMut::new(
			inner.or_insert_with(|| T::from_fn(shape.capacity(), |_| Default::default())),
			shape,
		)
	}
	pub fn value_mut(&mut self) -> Option<ChunkMut<T, S, B>> {
		self.inner
			.value_mut()
			.map(|storage| ChunkMut::new(storage, self.shape))
	}
	pub fn position(&self) -> math::Point<i32, C> {
		self.inner.position()
	}
	pub fn or_insert(self, default: T) -> ChunkMut<'a, T, S, B> {
		ChunkMut::new(self.inner.or_insert(default), self.shape)
	}
	pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> ChunkMut<'a, T, S, B> {
		ChunkMut::new(self.inner.or_insert_with(default), self.shape)
	}
	pub fn or_insert_with_key<F: FnOnce(math::Point<i32, C>) -> T>(
		self,
		default: F,
	) -> ChunkMut<'a, T, S, B> {
		ChunkMut::new(self.inner.or_insert_with_key(default), self.shape)
	}
}
