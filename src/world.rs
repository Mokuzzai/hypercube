use std::collections::BTreeMap;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::BlockPosition;
use crate::ChunkPosition;
use crate::Coordinate;

use crate::Chunk;

#[derive(Default)]
pub struct World<Cc: Coordinate, C, const D: usize> {
	chunks: BTreeMap<ChunkPosition<Cc, D>, C>,
}

impl<Cc: Coordinate + Ord, C, const D: usize> Deref for World<Cc, C, D> {
	type Target = BTreeMap<ChunkPosition<Cc, D>, C>;

	fn deref(&self) -> &Self::Target {
		&self.chunks
	}
}

impl<Cc: Coordinate + Ord, C, const D: usize> DerefMut for World<Cc, C, D> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.chunks
	}
}
