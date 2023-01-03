/*
use crate::*;

use std::collections::BTreeMap as Map;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OrderedChunkCoordinate {
	z: i32,
	y: i32,
	x: i32,
}

impl From<ChunkCoordinate> for OrderedChunkCoordinate {
	fn from(position: ChunkCoordinate) -> Self {
		Self {
			z: position.z,
			y: position.y,
			x: position.y,
		}
	}
}

pub struct World {
	chunks: Map<OrderedChunkCoordinate, Chunk>,
}

/// Chunk manipulation
impl World {
	pub fn chunk(&self, position: ChunkCoordinate) -> Option<&Chunk> {
		self.chunks.get(&OrderedChunkCoordinate::from(position))
	}
	pub fn chunk_mut(&mut self, position: ChunkCoordinate) -> Option<&mut Chunk> {
		self.chunks.get_mut(&OrderedChunkCoordinate::from(position))
	}
	pub fn chunk_or_insert(
		&mut self,
		position: ChunkCoordinate,
		chunk: impl FnOnce() -> Chunk,
	) -> &mut Chunk {
		self.chunks
			.entry(OrderedChunkCoordinate::from(position))
			.or_insert_with(chunk)
	}
	pub fn chunks(&self) -> impl Iterator<Item = &Chunk> {
		self.chunks.values()
	}
	pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
		self.chunks.values_mut()
	}
}

/// Block manipulation
impl World {
	pub fn block(&self, position: WorldCoordinate) -> Option<&Block> {
		Some(self.chunk(*position.chunk())?.get(*position.block()))
	}
	pub fn block_mut(&mut self, position: WorldCoordinate) -> Option<&mut Block> {
		Some(
			self.chunk_mut(*position.chunk())?
				.get_mut(*position.block()),
		)
	}

	// FIXME: when ever would you iterate over all the blocks?
	// pub fn blocks(&self) -> impl Iterator<Item = &T> {
	// 	self.chunks().flat_map(Chunk::blocks)
	// }
	// pub fn blocks_mut(&mut self) -> impl Iterator<Item = &mut T> {
	// 	self.chunks_mut().flat_map(Chunk::blocks_mut)
	// }
}*/
