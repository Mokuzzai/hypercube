use super::*;

/// Extends a [`Chunk`] with some payload
#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct WithPayload<C, P> {
	pub chunk: C,
	pub payload: P,
}

impl<C, P> WithPayload<C, P> {
	pub fn new(chunk: C, payload: P) -> Self {
		Self { chunk, payload }
	}
}

impl<C: Chunk<B>, P, const B: usize> Chunk<B> for WithPayload<C, P> {
	type Item = C::Item;
	type Shape = C::Shape;
	fn as_slice(&self) -> &[Self::Item] {
		self.chunk.as_slice()
	}
	fn as_mut_slice(&mut self) -> &mut [Self::Item] {
		self.chunk.as_mut_slice()
	}
	fn shape(&self) -> Cow<Self::Shape> {
		self.chunk.shape()
	}

	// NOTE: we implement theese methods manually because `C` may overload them
	fn get(&self, position: math::Point<i32, B>) -> Option<&Self::Item> {
		self.chunk.get(position)
	}
	fn get_mut(&mut self, position: math::Point<i32, B>) -> Option<&mut Self::Item> {
		self.chunk.get_mut(position)
	}
	fn get_replace(&mut self, position: math::Point<i32, B>, with: Self::Item) -> Option<Self::Item> {
		self.chunk.get_replace(position, with)
	}
	fn block(&self, position: math::Point<i32, B>) -> &Self::Item {
		self.chunk.block(position)
	}
	fn block_mut(&mut self, position: math::Point<i32, B>) -> &mut Self::Item {
		self.chunk.block_mut(position)
	}
	fn replace(&mut self, position: math::Point<i32, B>, with: Self::Item) -> Self::Item {
		self.chunk.replace(position, with)
	}
	fn positions(&self) -> Positions<B> {
		self.chunk.positions()
	}

	// NOTE: theese cannot be overloaded meaningfully
	// fn item_positions(&self) -> ItemsPositions<Self::Item, B> { .. }
	// fn item_positions_mut(&mut self) -> ItemsPositionsMut<Self::Item, B> { .. }
}
