mod with_payload;

pub use with_payload::WithPayload;

use crate::lazy_panic;
use crate::math;
use crate::Cow;
use crate::Positions;
use crate::Shape;

use std::slice;

pub trait Chunk<const B: usize> {
	type Item;
	type Shape: Shape<B>;

	fn shape(&self) -> Cow<Self::Shape>;

	fn as_slice(&self) -> &[Self::Item];
	fn as_mut_slice(&mut self) -> &mut [Self::Item];

	fn get(&self, position: math::Point<i32, B>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.as_slice().get(index)
	}
	fn get_mut(&mut self, position: math::Point<i32, B>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.as_mut_slice().get_mut(index)
	}
	fn get_replace(&mut self, position: math::Point<i32, B>, with: Self::Item) -> Option<Self::Item> {
		Some(std::mem::replace(self.get_mut(position)?, with))
	}
	fn block(&self, position: math::Point<i32, B>) -> &Self::Item {
		let extents = self.shape().extents();

		self.get(position).unwrap_or_else(lazy_panic!("position: `{:?}` out of bounds ({:?})", position, extents))
	}
	fn block_mut(&mut self, position: math::Point<i32, B>) -> &mut Self::Item {
		let extents = self.shape().extents();

		self.get_mut(position).unwrap_or_else(lazy_panic!("position: `{:?}` out of bounds ({:?})", position, extents))
	}
	fn replace(&mut self, position: math::Point<i32, B>, with: Self::Item) -> Self::Item {
		std::mem::replace(self.block_mut(position), with)
	}
	fn positions(&self) -> Positions<B> {
		self.shape().positions()
	}
	fn item_positions(&self) -> ItemPositions<Self::Item, B> {
		ItemPositions {
			extents: self.shape().extents(),
			inner: self.as_slice().iter().enumerate(),
		}
	}
	fn item_positions_mut(&mut self) -> ItemPositionsMut<Self::Item, B> {
		ItemPositionsMut {
			extents: self.shape().extents(),
			inner: self.as_mut_slice().iter_mut().enumerate(),
		}
	}
}

use std::iter::Enumerate;

#[derive(Debug)]
pub struct ItemPositions<'a, I, const B: usize> {
	extents: math::Vector<usize, B>,
	inner: Enumerate<slice::Iter<'a, I>>,
}

impl<'a, I, const B: usize> Iterator for ItemPositions<'a, I, B> {
	type Item = (math::Point<i32, B>, &'a I);

	fn next(&mut self) -> Option<Self::Item> {
		let (index, item) = self.inner.next()?;

		Some((math::index_to_position(self.extents, index)?, item))
	}
}

#[derive(Debug)]
pub struct ItemPositionsMut<'a, I, const B: usize> {
	extents: math::Vector<usize, B>,
	inner: Enumerate<slice::IterMut<'a, I>>,
}

impl<'a, I, const B: usize> Iterator for ItemPositionsMut<'a, I, B> {
	type Item = (math::Point<i32, B>, &'a mut I);

	fn next(&mut self) -> Option<Self::Item> {
		let (index, item) = self.inner.next()?;

		Some((math::index_to_position(self.extents, index)?, item))
	}
}
