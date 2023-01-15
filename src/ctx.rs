//! A 'Context' represents the neighboring chunks blocks which are directly next to our chunk
//! NOTE: Currently only supported for uniform chunks

use crate::Chunk;

pub struct DiagonalAxisContext<T> {
	pub near: T,
	pub far: T,
}

/// `C`: Chunkspace coordinates
/// `A`: Area of a single face
pub struct ParallelAxisContext<T, const A: usize> {
	pub near: [T; A],
	pub far: [T; A],
}

/// `C`: Chunkspace coordinates
/// `A`: Area of a single face
pub struct ParallelContext<T, const A: usize, const C: usize> {
	pub context: [ParallelAxisContext<T, A>; C],
}

/// `C`: Chunkspace coordinates
pub struct DiagonalContext<T, const C: usize> {
	pub context: [DiagonalAxisContext<T>; C],
}



