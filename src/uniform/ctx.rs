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
pub struct ParallelAxisContext<T, const STRIDE: usize, const SUBDIM: usize, const CAPACITY: usize> {
	pub near: UniformChunk<T, STRIDE, SUBDIM, CAPACITY>,
	pub far: UniformChunk<T, STRIDE, SUBDIM, CAPACITY>,
}

impl<T, const STRIDE: usize, const SUBDIM: usize, const CAPACITY: usize> ParallelAxisContext<T, STRIDE, SUBDIM, CAPACITY> {

}

#[derive(Debug)]
pub struct ParallelContext<T, const STRIDE: usize, const DIM: usize, const SUBDIM: usize, const CAPACITY: usize> {
	pub context: [ParallelAxisContext<T, STRIDE, SUBDIM, CAPACITY>; DIM],
}

