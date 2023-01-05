use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::ops::DerefMut;

use nalgebra::Point;
use nalgebra::Scalar;

use bytemuck::Pod;
use bytemuck::Zeroable;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct ChunkPosition<T, const D: usize> {
	pub coordinates: [T; D],
}

impl<T: Pod + Scalar, const D: usize> Deref for ChunkPosition<T, D> {
	type Target = Point<T, D>;

	fn deref(&self) -> &Self::Target {
		bytemuck::cast_ref(self)
	}
}

impl<T: Pod + Scalar, const D: usize> DerefMut for ChunkPosition<T, D> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		bytemuck::cast_mut(self)
	}
}

impl<T: PartialOrd, const D: usize> PartialOrd for ChunkPosition<T, D> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.try_fold(Ordering::Equal, |acc, (a, b)| {
				Some(acc.then(a.partial_cmp(b)?))
			})
	}
}

impl<T: Ord, const D: usize> Ord for ChunkPosition<T, D> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.coordinates
			.iter()
			.rev()
			.zip(other.coordinates.iter().rev())
			.fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp(b)))
	}
}

impl<T: Default, const D: usize> Default for ChunkPosition<T, D> {
	fn default() -> Self {
		Self {
			coordinates: std::array::from_fn(|_| T::default()),
		}
	}
}

#[derive(Default)]
pub struct World<Cc, C, const D: usize> {
	chunks: BTreeMap<ChunkPosition<Cc, D>, C>,
}

impl<Cc: Ord, C, const D: usize> Deref for World<Cc, C, D> {
	type Target = BTreeMap<ChunkPosition<Cc, D>, C>;

	fn deref(&self) -> &Self::Target {
		&self.chunks
	}
}

impl<Cc: Ord, C, const D: usize> DerefMut for World<Cc, C, D> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.chunks
	}
}
