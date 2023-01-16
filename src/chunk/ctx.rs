
use super::*;

use nd::Dimension;

pub type WithParallelContext2<T> = WithParallelContext<T, 2, 1>;
pub type WithParallelContext3<T> = WithParallelContext<T, 3, 2>;

// #[derive(Debug)]
pub struct WithParallelContext<T, const B: usize, const S: usize> {
	array: nd::Array<T, B>,
}

impl<T, const B: usize, const S: usize> Chunk<B> for WithParallelContext<T, B, S> {
	type Block = T;

	fn array(&self) -> &nd::Array<Self::Block, B> { &self.array }
	fn array_mut(&mut self) -> &mut nd::Array<Self::Block, B> { &mut self.array }
}

#[derive(Debug)]
pub struct ParallelContext<T, const B: usize, const S: usize> {
	ctx: Box<[T]>,

	stride: usize,
}

impl<T, const B: usize, const S: usize> ParallelContext<T, B, S>
where
	nd::Shape<B>: Dimension,
	nd::Shape<S>: Dimension,
{
	pub const SUBCOUNT: usize = B * 2;

	pub fn subshape(&self) -> nd::Shape<S> {
		[self.stride; S]
	}
	pub fn subsize(&self) -> usize {
		self.subshape().size()
	}
	pub fn subcount(&self) -> usize {
		Self::SUBCOUNT
	}
	pub fn sub(&self, index: usize) -> nd::ArrayView<T, S> {
		assert!(S == B - 1, "`S` must be equal to `B - 1`");
		assert!(index < self.subcount(), "index: `{}` of range `{:?}`: no such face", index, 0..self.subcount());

		let size = self.subsize();

		let start = index * size;
		let end = start + size;

		nd::ArrayView::<T, S>::from_shape([self.stride; S], &self.ctx[start..end]).unwrap()
	}
	pub fn sub_mut(&mut self, index: usize) -> nd::ArrayViewMut<T, S> {
		assert!(S == B - 1, "`S` must be equal to `B - 1`");
		assert!(index < self.subcount(), "index: `{}` of range `{:?}`: no such face", index, 0..self.subcount());

		let size = self.subsize();

		let start = index * size;
		let end = start + size;

		nd::ArrayViewMut::<T, S>::from_shape([self.stride; S], &mut self.ctx[start..end]).unwrap()
	}
}
