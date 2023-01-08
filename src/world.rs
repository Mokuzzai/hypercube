use std::collections::BTreeMap;
// use std::ops::Deref;
// use std::ops::DerefMut;

use crate::BlockPosition;
use crate::ChunkPosition;
use crate::Coordinate;

use crate::Chunk;

#[derive(Default)]
pub struct World<
	T,
	Cc: Coordinate,
	Scc: Coordinate,
	const S: usize,
	const D: usize,
	const C: usize,
	const D2: usize,
> {
	chunks: BTreeMap<ChunkPosition<Cc, D2>, Chunk<T, Scc, S, D, C>>,
}

pub type World16d2<T> = World<T, i32, u8, 16, 2, 256, 2>;
pub type World16d3<T> = World<T, i32, u8, 16, 3, 4096, 3>;
pub type World32d2<T> = World<T, i32, u8, 32, 2, 1024, 2>;
pub type World32d3<T> = World<T, i32, u8, 32, 2, 32768, 3>;

// impl<T, Cc: Coordinate, Ssc: Coordinate, const CS: usize, const SD: usize, const SC: usize, const D: usize> Deref for World<T, Cc, Ssc, CS, SD, SC, D> {
// 	type Target = BTreeMap<ChunkPosition<Cc, D>, Chunk<T, Ssc, CS, SD, SC>>;
//
// 	fn deref(&self) -> &Self::Target {
// 		&self.chunks
// 	}
// }
//
// impl<T, Cc: Coordinate, Ssc: Coordinate, const CS: usize, const SD: usize, const SC: usize, const D: usize> DerefMut for World<T, Cc, Ssc, CS, SD, SC, D> {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.chunks
// 	}
// }

impl<
		T,
		Cc: Coordinate + Ord,
		Scc: Coordinate,
		const S: usize,
		const D: usize,
		const C: usize,
		const D2: usize,
	> World<T, Cc, Scc, S, D, C, D2>
{
	pub fn chunk(&self, position: ChunkPosition<Cc, D2>) -> Option<&Chunk<T, Scc, S, D, C>> {
		self.chunks.get(&position)
	}
	pub fn chunk_mut(
		&mut self,
		position: ChunkPosition<Cc, D2>,
	) -> Option<&mut Chunk<T, Scc, S, D, C>> {
		self.chunks.get_mut(&position)
	}
	pub fn block(&mut self, position: BlockPosition<Cc, Scc, D, D2>) -> Option<&T> {
		self.chunk(position.chunk)?.get(position.sub_chunk)
	}
	pub fn block_mut(&mut self, position: BlockPosition<Cc, Scc, D, D2>) -> Option<&mut T> {
		self.chunk_mut(position.chunk)?.get_mut(position.sub_chunk)
	}
	pub fn iter(&self) -> impl Iterator<Item = (&ChunkPosition<Cc, D2>, &Chunk<T, Scc, S, D, C>)> {
		self.chunks.iter()
	}
	pub fn iter_mut(
		&mut self,
	) -> impl Iterator<Item = (&ChunkPosition<Cc, D2>, &mut Chunk<T, Scc, S, D, C>)> {
		self.chunks.iter_mut()
	}
}
