
mod with_payload;

pub use with_payload::WithPayload;

use crate::math;
use crate::Positions;
use crate::Shape;
use crate::Cow;

use std::slice;

pub trait Chunk<const B: usize> {
	type Item;
	type Shape: Shape<B>;

	fn shape(&self) -> Cow<Self::Shape>;

	fn as_slice(&self) -> &[Self::Item];
	fn as_mut_slice(&mut self) -> &mut [Self::Item];

	fn get(&self, position: math::Vector<i32, B>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.as_slice().get(index)
	}
	fn get_mut(&mut self, position: math::Vector<i32, B>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.as_mut_slice().get_mut(index)
	}
	fn replace(&mut self, position: math::Vector<i32, B>, with: Self::Item) -> Option<Self::Item> {
		Some(std::mem::replace(self.get_mut(position)?, with))
	}
	fn positions(&self) -> Positions<B> {
		self.shape().positions()
	}
	fn item_positions(&self) -> ItemsPositions<Self::Item, B> {
		ItemsPositions { extents: self.shape().extents(), inner: self.as_slice().iter().enumerate() }
	}
	fn item_positions_mut(&mut self) -> ItemsPositionsMut<Self::Item, B> {
		ItemsPositionsMut { extents: self.shape().extents(), inner: self.as_mut_slice().iter_mut().enumerate() }
	}
}

use std::iter::Enumerate;

#[derive(Debug)]
pub struct ItemsPositions<'a, I, const B: usize> {
	extents: math::Vector<usize, B>,
	inner: Enumerate<slice::Iter<'a, I>>,
}

impl<'a, I, const B: usize> Iterator for ItemsPositions<'a, I, B> {
	type Item = (math::Vector<i32, B>, &'a I);

	fn next(&mut self) -> Option<Self::Item> {
		let (index, item) = self.inner.next()?;

		Some((math::index_to_position(self.extents, index)?, item))
	}
}

#[derive(Debug)]
pub struct ItemsPositionsMut<'a, I, const B: usize> {
	extents: math::Vector<usize, B>,
	inner: Enumerate<slice::IterMut<'a, I>>,
}

impl<'a, I, const B: usize> Iterator for ItemsPositionsMut<'a, I, B> {
	type Item = (math::Vector<i32, B>, &'a mut I);

	fn next(&mut self) -> Option<Self::Item> {
		let (index, item) = self.inner.next()?;

		Some((math::index_to_position(self.extents, index)?, item))
	}
}

