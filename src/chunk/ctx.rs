
use super::*;

use nd::Dimension;

pub type WithParallelContext2<T> = WithParallelContext<T, 2, 1>;
pub type WithParallelContext3<T> = WithParallelContext<T, 3, 2>;

pub struct WithParallelContext<T, const B: usize, const S: usize> {
	array: nd::Array<T, B>,
}

impl<T, const B: usize, const S: usize> Chunk<B> for WithParallelContext<T, B, S> {
	type Block = T;

	fn array(&self) -> &nd::Array<Self::Block, B> { &self.array }
	fn array_mut(&mut self) -> &mut nd::Array<Self::Block, B> { &mut self.array }
}


pub struct ParallelContext<T, const B: usize, const S: usize> {
	ctx: Box<[T]>,

	stride: usize,
}

impl<T, const B: usize, const S: usize> ParallelContext<T, B, S>
where
	nd::Shape<B>: Dimension,
	nd::Shape<S>: Dimension,
{
	pub const FACE_COUNT: usize = B * 2;

	pub fn face_shape(&self) -> nd::Shape<B> {
		[self.stride; B]
	}
	pub fn face_size(&self) -> usize {
		self.face_shape().size()
	}
	pub fn face_count(&self) -> usize {
		Self::FACE_COUNT
	}
	pub fn view(&self, index: usize) -> nd::ArrayView<T, S> {
		assert!(S == B - 1, "`S` must be equal to `B - 1`");
		assert!(index < self.face_count(), "index: `{}` of range `{:?}`: no such face", index, 0..self.face_count());

		let size = self.face_size();

		let start = index * size;
		let end = start + size;

		nd::ArrayView::<T, S>::from_shape([self.stride; S], &self.ctx[start..end]).unwrap()
	}
	pub fn view_mut(&mut self, index: usize) -> nd::ArrayViewMut<T, S> {
		assert!(S == B - 1, "`S` must be equal to `B - 1`");
		assert!(index < self.face_count(), "index: `{}` of range `{:?}`: no such face", index, 0..self.face_count());

		let size = self.face_size();

		let start = index * size;
		let end = start + size;

		nd::ArrayViewMut::<T, S>::from_shape([self.stride; S], &mut self.ctx[start..end]).unwrap()
	}
}
