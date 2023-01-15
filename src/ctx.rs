//! A 'Context' represents the neighboring chunks blocks which are directly next to our chunk
//! NOTE: Currently only supported for uniform chunks

use crate::uniform::UniformChunk;
use crate::Chunk;
// #[derive(Debug)]
// pub struct DiagonalAxisContext<T> {
// 	pub near: T,
// 	pub far: T,
// }
//
// /// `C`: Chunkspace coordinates
// #[derive(Debug)]
// pub struct DiagonalContext<T, const C: usize> {
// 	pub context: [DiagonalAxisContext<T>; C],
// }

#[derive(Debug)]
pub struct ParallelAxisContext<T, const STIDE: usize, const SUBDIM: usize, const CAPACITY: usize> {
	pub near: UniformChunk<T, STIDE, SUBDIM, CAPACITY>,
	pub far: UniformChunk<T, STIDE, SUBDIM, CAPACITY>,
}

impl<T, const STIDE: usize, const SUBDIM: usize, const CAPACITY: usize> ParallelAxisContext<T, STIDE, SUBDIM, CAPACITY> {
	fn from_chunk<U, const DIM: usize>(chunk: &U, axis: usize) -> Self
	where
		T: Copy,
		U: Chunk<DIM>,
		U::Item: AsRef<T>,
	{
		todo!()
	}
}

#[derive(Debug)]
pub struct ParallelContext<T, const STRIDE: usize, const DIM: usize, const SUBDIM: usize, const CAPACITY: usize> {
	pub context: [ParallelAxisContext<T, STRIDE, SUBDIM, CAPACITY>; DIM],
}

impl<T, const STRIDE: usize, const DIM: usize, const SUBDIM: usize, const CAPACITY: usize> ParallelContext<T, STRIDE, DIM, SUBDIM, CAPACITY>  {
	fn from_chunk<U>(chunk: &U) -> Self
	where
		T: Copy,
		U: Chunk<DIM>,
		U::Item: AsRef<T>,
	{
		todo!()
	}
}
