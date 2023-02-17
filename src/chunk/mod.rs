
mod with_payload;

pub use with_payload::WithPayload;

use crate::na;
use crate::Positions;
use crate::Shape;
use crate::Cow;

pub trait Chunk<const B: usize> {
	type Item;
	type Shape: Shape<B>;

	fn shape(&self) -> Cow<Self::Shape>;

	fn index(&self, index: usize) -> Option<&Self::Item>;
	fn index_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

	fn get(&self, position: na::Vector<i32, B>) -> Option<&Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index(index)
	}
	fn get_mut(&mut self, position: na::Vector<i32, B>) -> Option<&mut Self::Item> {
		let index = self.shape().position_to_index(position)?;

		self.index_mut(index)
	}
	fn replace(&mut self, position: na::Vector<i32, B>, with: Self::Item) -> Option<Self::Item> {
		Some(std::mem::replace(self.get_mut(position)?, with))
	}
	fn positions(&self) -> Positions<B> {
		self.shape().positions()
	}
}

